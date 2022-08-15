extern crate handlebars;

use handlebars::{Handlebars as Registry, JsonValue};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use wasm_bindgen::prelude::*;

mod js_helper;
mod utils;
use js_helper::{JsHelper, Tags};

#[derive(Serialize, Deserialize)]
pub struct Config {
  tags: Option<Tags>,
}

#[wasm_bindgen]
pub struct Handlebars {
  instance: Registry<'static>,
  tags: Tags,
}

#[allow(clippy::new_without_default)]
#[wasm_bindgen]
impl Handlebars {
  #[wasm_bindgen(constructor)]
  pub fn new(config: &JsValue) -> Self {
    let config: Config = config.into_serde().unwrap();
    let tags = match config.tags {
      Some(tags) => tags,
      None => Tags {
        template: "$$TEMPLATE$$".to_string(),
        inverse: "$$INVERSE$$".to_string(),
      },
    };

    Handlebars {
      instance: Registry::new(),
      tags,
    }
  }

  pub fn compile(&self, tpl: String, data: &[u8]) -> String {
    let json = from_slice::<JsonValue>(data).unwrap();
    let result = match self.instance.render_template(tpl.as_str(), &json) {
      Ok(tpl_string) => tpl_string,
      Err(error) => panic!("Compile template error: {:?}", error),
    };

    result
  }

  pub fn register_helper(&mut self, name: String, wrapper: Function, f: Function) {
    self.instance.register_helper(
      name.as_str(),
      Box::new(JsHelper {
        js_fn_tpl: format!("return {}", f.to_string().as_string().unwrap()),
        wrap_fn_tpl: format!("return {}", wrapper.to_string().as_string().unwrap()),
        tags: Tags {
          template: self.tags.template.to_string(),
          inverse: self.tags.inverse.to_string(),
        },
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
