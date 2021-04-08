extern crate napi;
#[macro_use]
extern crate napi_derive;

use napi::{CallContext, Result, JsObject, JsNumber };

use std::convert::TryInto;

#[module_exports]
fn init(mut exports: JsObject ) -> Result<()> {
  exports.create_named_method("add", add)?;
  Ok(())
}

#[js_function(2)] // ------> arguments length, omit for zero
fn add(ctx: CallContext) -> Result<JsNumber> {
    let a: f64 = ctx.get::<JsNumber>(0)?.try_into().unwrap();
    let b: f64 = ctx.get::<JsNumber>(1)?.try_into().unwrap();
    ctx.env.create_double(a + b)
}
