use handlebars::{BlockContext, BlockParams, Helper, PathAndJson, RenderError};
use serde_json::value::Value as Json;

#[inline]
pub(crate) fn copy_on_push_vec<T>(input: &[T], el: T) -> Vec<T>
where
  T: Clone,
{
  let mut new_vec = Vec::with_capacity(input.len() + 1);
  new_vec.extend_from_slice(input);
  new_vec.push(el);
  new_vec
}

pub(crate) fn create_block<'reg: 'rc, 'rc>(
  param: &'rc PathAndJson<'reg, 'rc>,
) -> BlockContext<'reg> {
  let mut block = BlockContext::new();

  block.set_base_value(param.value().clone());
  block
}

pub fn update_block_context<'reg>(
  block: &mut BlockContext<'reg>,
  base_path: Option<&Vec<String>>,
  relative_path: String,
  is_first: bool,
  value: &Json,
) {
  // console::log_1(&JsValue::from_serde(value).unwrap());
  // if let Some(p) = base_path {
  //   if is_first {
  //     *block.base_path_mut() = copy_on_push_vec(p, relative_path);
  //   } else if let Some(ptr) = block.base_path_mut().last_mut() {
  //     *ptr = relative_path;
  //   }
  // } else {
  //   block.set_base_value(value.clone());
  // }
  block.set_base_value(value.clone());
}

pub fn set_block_param<'reg: 'rc, 'rc>(
  block: &mut BlockContext<'reg>,
  h: &Helper<'reg, 'rc>,
  base_path: Option<&Vec<String>>,
  k: &Json,
  v: &Json,
) -> Result<(), RenderError> {
  if let Some(bp_val) = h.block_param() {
    let mut params = BlockParams::new();
    if base_path.is_some() {
      params.add_path(bp_val, Vec::with_capacity(0))?;
    } else {
      params.add_value(bp_val, v.clone())?;
    }

    block.set_block_params(params);
  } else if let Some((bp_val, bp_key)) = h.block_param_pair() {
    let mut params = BlockParams::new();
    if base_path.is_some() {
      params.add_path(bp_val, Vec::with_capacity(0))?;
    } else {
      params.add_value(bp_val, v.clone())?;
    }
    params.add_value(bp_key, k.clone())?;

    block.set_block_params(params);
  }

  Ok(())
}
