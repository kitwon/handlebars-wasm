extern crate handlebars;

use handlebars::{Handlebars as Registry, JsonValue};
use js_sys::Function;
use serde_json::from_slice;
use wasm_bindgen::prelude::*;

mod js_helper;
use js_helper::JsHelper;

#[wasm_bindgen]
pub struct Handlebars {
  instance: Registry<'static>,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Handlebars {
  #[wasm_bindgen(constructor)]
  pub fn new() -> Self {
    Handlebars {
      instance: Registry::new(),
    }
  }

  pub fn compile(&self, tpl: String, data: &[u8]) -> String {
    let json = from_slice::<JsonValue>(data).unwrap();
    let result = match self.instance.render_template(tpl.as_str(), &json) {
      Ok(tpl_string) => tpl_string,
      Err(error) => panic!("Compile templete error: {:?}", error),
    };

    result
  }

  pub fn register_helper(&mut self, name: String, wrapper: Function, f: Function) {
    self.instance.register_helper(
      name.as_str(),
      Box::new(JsHelper {
        js_fn_tpl: format!("return {}", f.to_string().as_string().unwrap()),
        wrap_fn_tpl: format!("return {}", wrapper.to_string().as_string().unwrap()),
      }),
    )
  }

  pub fn register_partial(&mut self, name: String, partial: String) {
    self
      .instance
      .register_partial(name.as_str(), partial)
      .unwrap()
  }
}
