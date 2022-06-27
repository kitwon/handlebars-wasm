use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};
use js_sys::Function;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct HelperOptions {
    template_ctx: JsValue,
    inverse_ctx: JsValue,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl HelperOptions {
    pub fn new() -> HelperOptions {
        HelperOptions {
            template_ctx: JsValue::undefined(),
            inverse_ctx: JsValue::undefined(),
        }
    }

    pub fn template(&mut self, ctx: JsValue) -> String {
        self.template_ctx = ctx;
        "$$TEMPLATE$$".to_string()
    }

    pub fn inverse(&mut self, ctx: JsValue) -> String {
        self.inverse_ctx = ctx;
        "$$INVERSE$$".to_string()
    }
}

pub struct JsHelper {
    pub js_fn_tpl: String,
}

impl HelperDef for JsHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _: &'reg Handlebars<'reg>,
        _: &'rc Context,
        _: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        // Change function template to actual js function
        let js_fn = Function::new_no_args(&self.js_fn_tpl);
        let js_helper = Function::from(js_fn.call0(&JsValue::null()).unwrap());

        let value = h
            .param(0)
            .ok_or_else(|| RenderError::new("Param not found in js helper"))
            .unwrap();

        let data = JsValue::from_serde(value.value()).unwrap();

        let options = HelperOptions::new();
        let result = js_helper
            .call2(&JsValue::null(), &data, &options.into())
            .unwrap();
        let result = result.as_string().unwrap();
        out.write(result.as_str())?;
        Ok(())
    }
}
