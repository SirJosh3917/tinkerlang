use std::sync::Arc;

use inkwell::{context::Context, module::Module};

use super::js_boundary::JsMetaHandle;

pub fn hydrate(meta: &JsMetaHandle, context: &Context, module: &mut Module) {}
