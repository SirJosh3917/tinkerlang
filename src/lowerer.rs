use quick_js::{Context, ExecutionError, JsValue};
use std::{
    collections::HashMap,
    fmt::{self, Write},
    ops::Range,
};
use tree_sitter::{Tree, TreeCursor};

use crate::ir::IrBuilder;

pub struct Lowerer {
    context: Context,
    ir_builder: IrBuilder,
}

impl Lowerer {
    pub fn new<F: FnOnce(&mut Context) -> IrBuilder>(
        source: &str,
        tree: Tree,
        ir_builder_factory: F,
    ) -> Self {
        let mut context = Context::builder()
            .console(|level, args: Vec<JsValue>| {
                let mut arg_builder = String::new();
                for arg in args.iter() {
                    let result = write!(arg_builder, "{}", DisplayJsValue(arg));

                    #[cfg(debug_assertions)]
                    result.expect("expected to write to string");
                }

                println!("{}: {}", level, arg_builder);
            })
            .build()
            .expect("asdf");

        // prime the context
        context
            .set_global("tree", alloc_to_js_heap(tree, source))
            .expect("expected to add global `tree`");

        let source_clone = source.to_owned();
        context
            .add_callback("toValue", move |range: f64| {
                let range = f64_to_range(range);
                source_clone[range].to_owned()
            })
            .expect("expected to add callback `toValue`");

        let ir_builder = ir_builder_factory(&mut context);

        Lowerer {
            context,
            ir_builder,
        }
    }

    pub fn exec(&self, lowerer_src: &str) -> Result<JsValue, ExecutionError> {
        self.context.eval(lowerer_src)
    }

    pub fn make_llvm<'ctx>(
        &self,
        context: &'ctx inkwell::context::Context,
    ) -> inkwell::module::Module<'ctx> {
        let mut module = context.create_module("tinkerlang_module");
        self.ir_builder.hydrate(&context, &mut module);
        module
    }
}

struct DisplayJsValue<'a>(&'a JsValue);

impl<'a> fmt::Display for DisplayJsValue<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            JsValue::String(v) => write!(f, "{}", v),
            JsValue::Float(v) => write!(f, "{}", v),
            JsValue::Int(v) => write!(f, "{}", v),
            _ => unimplemented!(),
        }
    }
}

fn alloc_to_js_heap(tree: tree_sitter::Tree, input: &str) -> JsValue {
    let mut cursor = tree.root_node().walk();
    let tree = JsValue::Object(walk_recurse(input, &mut cursor));

    fn walk_recurse(src: &str, cursor: &mut TreeCursor) -> HashMap<String, JsValue> {
        let mut children = Vec::new();

        if cursor.goto_first_child() {
            loop {
                children.push(JsValue::Object(walk_recurse(src, cursor)));

                if !cursor.goto_next_sibling() {
                    break;
                }
            }

            let goto_parent = cursor.goto_parent();
            debug_assert!(goto_parent);
        }

        let mut map = HashMap::new();

        map.insert(
            "type".to_owned(),
            JsValue::String(cursor.node().kind().to_owned()),
        );

        let node = cursor.node();

        map.insert(
            "value".to_owned(),
            range_to_js(node.start_byte()..node.end_byte()),
        );

        map.insert("children".to_owned(), JsValue::Array(children));

        map
    }

    tree
}

fn range_to_js(range: Range<usize>) -> JsValue {
    let Range { start, end } = range;

    assert!(
        start < u32::MAX as usize,
        "expected start position to be less than {}",
        u32::MAX
    );
    assert!(
        end < u32::MAX as usize,
        "expected end position to be less than {}",
        u32::MAX
    );

    let start = start as u32;
    let end = end as u32;
    let range_in_64_bits = (start as u64) << 32 | end as u64;
    let range_as_fp = f64::from_ne_bytes(range_in_64_bits.to_ne_bytes());

    JsValue::Float(range_as_fp)
}

fn js_to_range(js: JsValue) -> Range<usize> {
    let float = match js {
        JsValue::Float(value) => value,
        _ => panic!("js_to_range expected float, got {:?}", js),
    };

    f64_to_range(float)
}

fn f64_to_range(float: f64) -> Range<usize> {
    let bytes = u64::from_ne_bytes(float.to_ne_bytes());

    let start = (bytes >> 32) as u32;
    let end = bytes as u32;

    start as usize..end as usize
}
