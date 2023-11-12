use crate::state::{SyncState, ID};
use neon::prelude::*;

pub fn js_number_new(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let id = state.number_new();
        return Ok(cx.number(id as f64));
    };
}

pub fn js_number_get(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let v = state.number_get(&id).unwrap();
        return Ok(cx.number(*v.clone()));
    };
}

pub fn js_number_set(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsNumber> = cx.argument(1)?;

        let id = arg0.value(&mut cx).floor() as ID;
        let update = arg1.value(&mut cx);

        let ok = state.number_set(&id, update);
        return Ok(cx.boolean(ok));
    };
}
