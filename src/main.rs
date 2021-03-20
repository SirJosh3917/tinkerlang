extern crate inkwell;

use std::marker::PhantomData;

use inkwell::targets::{CodeModel, FileType, InitializationConfig, RelocMode, Target};
use inkwell::values::BasicValue;
use inkwell::{context::Context, targets::TargetMachine};
use inkwell::{module::Linkage, OptimizationLevel};
use lld_sys::{llvm_ArrayRef, llvm_ArrayRef_size_type, llvm_raw_ostream};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
