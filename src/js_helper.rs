use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};
use js_sys::Function;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen::prelude::*;
use web_sys::console;

fn get_js_fn(tpl: &str) -> JsValue {
  let js_fn = Function::new_no_args(tpl);

  match js_fn.call0(&JsValue::null()) {
    Ok(val) => val,
    Err(error) => panic!("Transform js helper function error: {:?}", error),
  }
}

#[derive(Serialize, Deserialize)]
pub struct JsHelperCtxs {
  inverse: Vec<Value>,
  template: Vec<Value>,
}

#[derive(Serialize, Deserialize)]
pub struct JsHelperResult {
  text: String,
  ctxs: JsHelperCtxs,
}

/**
 * Option function tags
 */
#[derive(Serialize, Deserialize)]
pub struct Tags {
  pub template: String,
  pub inverse: String,
}

#[wasm_bindgen]
pub struct HelperOptions {
  template_tag: String,
  inverse_tag: String,
}

#[wasm_bindgen]
impl HelperOptions {
  pub fn template(&mut self) -> String {
    self.template_tag.clone()
  }

  pub fn inverse(&mut self) -> String {
    self.inverse_tag.clone()
  }
}

pub struct JsHelper {
  pub js_fn_tpl: String,
  pub wrap_fn_tpl: String,
  pub tags: Tags,
}

impl HelperDef for JsHelper {
  fn call<'reg: 'rc, 'rc>(
    &self,
    h: &Helper<'reg, 'rc>,
    r: &'reg Handlebars<'reg>,
    ctx: &'rc Context,
    rc: &mut RenderContext<'reg, 'rc>,
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

    let options = HelperOptions {
      inverse_tag: self.tags.inverse.to_string(),
      template_tag: self.tags.template.to_string(),
    };
    let result = match helper_wrapper.call3(&JsValue::null(), &data, &options.into(), &js_helper) {
      Ok(res) => res,
      Err(err) => {
        console::log_1(&err);
        JsValue::undefined()
      }
    };
    let result: JsHelperResult = result.into_serde().unwrap();
    println!("{:?}", result.ctxs.template);

    // let result = result
    //   .replace(self.tags.template.to_string(), Some(h.template()))
    //   .replace(self.tags.inverse.to_string(), h.inverse());
    out.write(result.text.as_str())?;
    Ok(())
  }
}

#[cfg(test)]
mod test {
  use crate::js_helper::{JsHelper, Tags};
  use handlebars::Handlebars;

  const WRAPPER: &str = "(data: any, options: HelperOption, h: A) => {
    let args = [options];
    if (data && Array.isArray(data)) {
      args = data.concat(args);
    } else if (data) {
      args.unshift(data);
    }

    return h(...args);
  };";

  #[test]
  fn test_js_iterator() {
    let mut hbs = Handlebars::new();
    hbs.register_helper(
      "list",
      Box::new(JsHelper {
        js_fn_tpl: "return (list: string[], options: HelperOption) => {
          return list
            .map((i) => {
              return options.fn(i);
            })
            .join('');
        }"
        .to_string(),
        wrap_fn_tpl: format!("return {}", WRAPPER),
        tags: Tags {
          inverse: "%%INVERSE%%".to_string(),
          template: "%%TEMPLATE%%".to_string(),
        },
      }),
    )
  }
}
