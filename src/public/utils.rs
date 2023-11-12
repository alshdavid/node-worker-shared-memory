use crate::state::{SyncState, ID};
use neon::prelude::*;

pub fn js_typeof(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsString> {
    return move |mut cx: FunctionContext| -> JsResult<JsString> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let result = state.type_of(&id);
        return Ok(cx.string(result));
    };
}

pub fn js_drop(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let ok = state.free_value(&id);
        return Ok(cx.boolean(ok));
    };
}
