use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use quick_js::JsValue;

pub type JsMetaHandle = Arc<Mutex<JsMeta>>;
pub struct JsMeta {
    types: Vec<TypeDefinition>,
}

pub struct TypeDefinition {
    signed: bool,
    bits: u32,
}

impl JsMeta {
    pub fn new() -> Self {
        JsMeta { types: Vec::new() }
    }
}

pub fn hook(context: &mut quick_js::Context) -> JsMetaHandle {
    let primer = include_str!("./primer.js");
    let source_meta = Arc::new(Mutex::new(JsMeta::new()));

    context
        .eval(primer)
        .expect("expected primer to inject okay");

    let meta = source_meta.clone();
    context
        .add_callback("__compiler_type", move |signed: bool, size: i32| {
            let mut meta = meta.lock().unwrap();

            if size < 0 {
                panic!("expected >= 0 bits");
            }

            let type_id = meta.types.len();
            meta.types.push(TypeDefinition {
                signed,
                bits: size as u32,
            });

            JsValue::Int(type_id as i32)
        })
        .expect("expected to inject __compiler_type");

    let meta = source_meta.clone();
    context
        .add_callback(
            "__compiler_generate_method",
            move |name: String, return_type: i32, parameters: Vec<i32>| {
                panic!();
                JsValue::Undefined
            },
        )
        .expect("expected to inject __compiler_generate_method");

    source_meta
}
