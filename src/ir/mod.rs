use self::js_boundary::JsMetaHandle;

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

    pub fn hydrate<'ctx>(
        &self,
        context: &'ctx inkwell::context::Context,
        module: &mut inkwell::module::Module<'ctx>,
    ) {
        emit::hydrate(&self.meta, context, module)
    }
}
