extern crate handlebars;

use handlebars::{Handlebars as Registry, JsonValue};
use js_sys::{Error, Function};
use serde_json::from_slice;
use wasm_bindgen::prelude::*;

mod js_helper;
mod utils;
use js_helper::JsHelper;
use web_sys::console;

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

  pub fn compile(&self, tpl: String, data: &[u8]) -> Result<String, Error> {
    let json = match from_slice::<JsonValue>(data) {
      Ok(data) => data,
      Err(error) => {
        console::log_1(&error.to_string().into());
        panic!("{:?}", error)
      }
    };
    // console::log_1(&JsValue::from_serde(&json).unwrap());

    let result = match self.instance.render_template(tpl.as_str(), &json) {
      Ok(tpl_string) => Ok(tpl_string),
      Err(error) => Err(Error::new(&error.to_string())),
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

#[cfg(test)]
mod test {
  use crate::Handlebars;
  use js_sys::Function;
  use wasm_bindgen::JsValue;
  use wasm_bindgen_test::wasm_bindgen_test;

  const WRAPPER: &str = "
    function wrapper(data, h) {
      const ctxs = {
        inverse: [],
        template: [],
      };

      const wrapOptionFn = (ctxs) => {
        return {
          template: (context) => {
            ctxs.template.push(context);
          },
          inverse(context) {
            ctxs.inverse.push(context);
          },
        };
      };

      let args = [data, wrapOptionFn(ctxs)];

      return { text: h(...args), ctxs };
    };
  ";

  #[wasm_bindgen_test]
  fn test_boolean() {
    let mut hb = Handlebars::new();
    hb.register_helper(
      String::from("goodbye"),
      Function::new_no_args(WRAPPER),
      Function::new_no_args("function helper() { return '' }"),
    );
    let result = hb.compile(
      "{{#goodbye}}GOODBYE {{/goodbye}}cruel {{world}}!".to_string(),
      "{
        \"goodbye\": true,
        \"world\": \"world\"
      }"
      .as_bytes(),
    );
    println!("{:?}", result.unwrap())
  }
}
