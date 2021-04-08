extern crate neon;

use neon::prelude::*;

fn add(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let a = cx.argument::<JsNumber>(0)?.value();
    let b = cx.argument::<JsNumber>(1)?.value();

    Ok(cx.number(a + b as f64))
}

register_module!(mut cx, {
    cx.export_function("add", add)
});
