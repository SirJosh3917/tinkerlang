use std::sync::Arc;

use self::js_boundary::{JsMeta, JsMetaHandle};

pub(crate) mod emit;
pub(crate) mod js_boundary;

pub struct IrBuilder {
    meta: JsMetaHandle,
}

impl IrBuilder {
    pub fn new(context: &mut quick_js::Context) -> Self {
        let meta = js_boundary::hook(context);
        Self { meta }
    }

    pub fn hydrate(
        &self,
        context: &inkwell::context::Context,
        module: &mut inkwell::module::Module,
    ) {
        emit::hydrate(&self.meta, context, module)
    }
}
