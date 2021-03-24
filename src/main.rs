#![feature(exclusive_range_pattern)]

extern crate inkwell;

use std::marker::PhantomData;

use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target};
use inkwell::values::BasicValue;
use inkwell::{context::Context, targets::TargetMachine};
use inkwell::{module::Linkage, OptimizationLevel};
use lld_sys::{llvm_ArrayRef_size_type, llvm_raw_ostream};
use lowerer::Lowerer;
use structopt::StructOpt;
use tree_sitter::{Language, Parser};

mod lowerer;

#[derive(Debug, StructOpt)]
#[structopt(name = "tinkerlang", about = "Prototype a programming language.")]
struct TinkerlangOptions {
    /// Name of the tree-sitter parser to use. This will automatically locate
    /// the parser in ~/.tree-sitter/bin/ for the parser. Case-sensittive.
    #[structopt(short, long)]
    parser: String,

    /// Path to the JS to execute. Currently, this is restricted to JS but can
    /// soon be expanded to writing lowerers in other languages. The lowerer is
    /// the part of the compiler that takes the tree-sitter AST and traverses
    /// over it to produce bytecode.
    #[structopt(short, long)]
    lowerer: String,

    /// The input to feed to the compiler. This will first pass through the
    /// tree-sitter parser, then be converted into a single AST tree in memory,
    /// then ran through the lowerer into Tinkerlang IR, then lowered into LLVM
    /// IR, then pushed through LLVM's pipeline and into an output binary.
    #[structopt(short, long)]
    input: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(debug_assertions)]
    let options = TinkerlangOptions {
        parser: "javascript".to_owned(),
        input: "../ex.js".to_owned(),
        lowerer: "../l.js".to_owned(),
    };
    #[cfg(not(debug_assertions))]
    let options = TinkerlangOptions::from_args();

    let language = load_language(options.parser.as_str());
    let input = std::fs::read_to_string(options.input).expect("expected to read input into string");

    let mut parser = Parser::new();
    parser
        .set_language(language)
        .expect("expected to set language");

    let tree = parser.parse(input.as_str(), None).unwrap();

    let lowerer = Lowerer::new(input.as_str(), tree);

    let lowerer_src =
        std::fs::read_to_string(options.lowerer).expect("expected to read lowerer into string");

    let result = lowerer
        .exec(lowerer_src.as_str())
        .expect("to run lowerer successfully");

    println!("got result: {:?}", result);

    println!(" --> rust:");
    let context = Context::create();
    let module = context.create_module("sum");
    let builder = context.create_builder();

    // ==> create adder function
    let i32_type = context.i32_type();
    let fn_type = i32_type.fn_type(&[], false);
    let function = module.add_function("main", fn_type, Some(Linkage::External));
    let basic_block = context.append_basic_block(function, "entry");

    builder.position_at_end(basic_block);

    let retval = i32_type.const_int(69, false);
    retval.set_name("retval");

    builder.build_return(Some(&retval));

    // ==> make binary
    Target::initialize_all(&InitializationConfig::default());

    let target_triple = TargetMachine::get_default_triple();
    let target = Target::from_triple(&target_triple).unwrap();
    let target_machine = target
        .create_target_machine(
            &target_triple,
            "generic",
            "",
            OptimizationLevel::Aggressive,
            RelocMode::Static,
            CodeModel::Default,
        )
        .expect("couldn't make target machine");

    target_machine
        .write_to_file(&module, FileType::Object, "ret69.o".as_ref())
        .expect("couldn't write to disk");

    println!(" --> lld:");
    // https://stackoverflow.com/a/30705769
    // ld.lld -L/usr/lib64 -dynamic-linker /lib64/ld-linux-x86-64.so.2 /usr/lib64/crt1.o /usr/lib64/crti.o -lc main.o /usr/lib64/crtn.o
    let args = vec![
        "ld.lld\0",
        "-L/usr/lib64\0",
        "-dynamic-linker\0",
        "/lib64/ld-linux-x86-64.so.2\0",
        "/usr/lib64/crt1.o\0",
        "/usr/lib64/crti.o\0",
        "-lc\0",
        "ret69.o\0",
        "/usr/lib64/crtn.o\0",
    ]
    .into_iter()
    .map(|s| s.as_ptr() as *const i8)
    .collect::<Vec<_>>();
    let args_llvm = lld_sys::llvm_ArrayRef {
        Data: args.as_ptr(),
        Length: args.len() as llvm_ArrayRef_size_type,
        _phantom_0: PhantomData,
    };

    unsafe {
        lld_sys::lld_elf_link(
            args_llvm,
            false,
            &mut (*lld_sys::llvm_outs())._base._base as *mut llvm_raw_ostream,
            &mut (*lld_sys::llvm_errs())._base._base as *mut llvm_raw_ostream,
        )
    };

    Ok(())
}

fn load_language(lang_name: &str) -> Language {
    let lib_path = dirs::home_dir()
        .unwrap()
        .join(format!(".tree-sitter/bin/{}.so", lang_name))
        .canonicalize()
        .unwrap();

    let parser_lib =
        unsafe { libloading::Library::new(lib_path) }.expect("expected to read parser library");

    let lang_fn_name = format!("tree_sitter_{}", lang_name.replace("-", "_"));

    let language = unsafe {
        parser_lib
            .get::<unsafe extern "C" fn() -> Language>(lang_fn_name.as_bytes())
            .expect("expected to load language symbol")()
    };

    std::mem::forget(parser_lib);

    language
}
