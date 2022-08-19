use crate::utils::{create_block, set_block_param};
use handlebars::{
  to_json, Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
  RenderError, Renderable,
};
use js_sys::{Array, Function};
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

fn get_js_params<'reg: 'rc, 'rc>(h: &Helper<'reg, 'rc>) -> Array {
  let data: Vec<JsValue> = h
    .params()
    .iter()
    .map(|v| match JsValue::from_serde(&v.value()) {
      Ok(value) => value,
      Err(_e) => JsValue::undefined(),
    })
    .collect();
  let js_array = Array::new();

  for i in data.iter() {
    js_array.push(i);
  }

  js_array
}

// TODO: make context mix inverse and template
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

pub struct JsHelper {
  pub js_fn_tpl: String,
  pub wrap_fn_tpl: String,
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
    let data = get_js_params(h);

    let result = match helper_wrapper.call2(&JsValue::null(), &data.into(), &js_helper) {
      Ok(res) => res,
      Err(err) => {
        console::log_1(&err);
        JsValue::undefined()
      }
    };

    match result.into_serde::<JsHelperResult>() {
      Ok(res) => {
        if let Some(t) = h.template() {
          let len = res.ctxs.template.len();
          if len > 0 {
            let value = h
              .param(0)
              .ok_or_else(|| console::log_1(&"Get helper params error".into()));

            let block_context = create_block(value.unwrap());
            rc.push_block(block_context);

            let array_path = value.unwrap().context_path();

            for (i, v) in res.ctxs.template.iter().enumerate().take(len) {
              if let Some(ref mut block) = rc.block_mut() {
                let is_first = i == 0usize;
                let is_last = i == len - 1;

                let index = to_json(i);
                block.set_local_var("first", to_json(is_first));
                block.set_local_var("last", to_json(is_last));
                block.set_local_var("index", index.clone());

                block.set_base_value(v.clone());
                set_block_param(block, h, array_path, &index, v)?;
              }

              t.render(r, ctx, rc, out)?;
            }
            rc.pop_block();
          }
        }

        out.write(res.text.as_str())?;
        Ok(())
      }

      Err(e) => {
        console::log_1(&"Parse js helper error".into());
        Err(RenderError::new(&"Parse js helper error"))
      }
    }
  }
}
