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
    let result = self.instance.render_template(tpl.as_str(), &json).unwrap();
    result
  }

  pub fn register_helper(&mut self, name: String, f: Function) {
    self.instance.register_helper(
      name.as_str(),
      Box::new(JsHelper {
        js_fn_tpl: format!("return {}", f.to_string().as_string().unwrap()),
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

#[cfg(test)]
mod test {
  use crate::Handlebars;
  use js_sys::Function;
  use wasm_bindgen_test::*;

  #[wasm_bindgen_test]
  fn helper() {
    let mut inst = Handlebars::new();
    inst.register_helper(
      "test".to_string(),
      Function::new_with_args(
        "a, option",
        "
      console.log(a, option)
      console.log(option.template)
      return `I'm arg: ${a}`
    ",
      ),
    );
    let result = inst.compile(
      "hello world! {{#test foo}}123{{/test}} {{bar}}".to_string(),
      "{
      \"foo\": \"bar\",
      \"bar\": \", but bar is foo\"
    }"
      .as_bytes(),
    );
    console_log!("{:?}", result);
  }
}
