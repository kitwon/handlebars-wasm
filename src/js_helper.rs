use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use js_sys::Function;
use wasm_bindgen::prelude::*;

fn get_js_fn(tpl: &str) -> JsValue {
  let js_fn = Function::new_no_args(tpl);

  match js_fn.call0(&JsValue::null()) {
    Ok(val) => val,
    Err(error) => panic!("Transform js helper function error: {:?}", error),
  }
}

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
  pub wrap_fn_tpl: String,
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
    let wrapper = get_js_fn(&self.wrap_fn_tpl);
    let helper = get_js_fn(&self.js_fn_tpl);

    let helper_wrapper = Function::from(wrapper);
    let js_helper = Function::from(helper);

    let value = h.param(0).unwrap();

    let data = match JsValue::from_serde(value.value()) {
      Ok(val) => val,
      Err(_) => JsValue::null(),
    };

    let options = HelperOptions::new();
    let result = match helper_wrapper.call3(&JsValue::null(), &data, &options.into(), &js_helper) {
      Ok(res) => res,
      Err(err) => panic!("Call helper function error: {:?}", err),
    };
    let result = result.as_string().unwrap();
    out.write(result.as_str())?;
    Ok(())
  }
}
