use crate::state::{SyncState, ID, matches_number, matches_string, matches_vector, StateType};
use neon::prelude::*;

pub fn js_typeof(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsString> {
    return move |mut cx: FunctionContext| -> JsResult<JsString> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let result_opt = state.values.get(&id);
        if result_opt.is_none() {
            return Ok(cx.string("undefined"));
        }

        let result = result_opt.unwrap();
        if matches_number(&result).is_ok() {
            return Ok(cx.string("number"));
        }
        if matches_string(&result).is_ok() {
            return Ok(cx.string("string"));
        }
        if matches_vector(&result).is_ok() {
            return Ok(cx.string("vector"));
        }

        panic!("unknown type");
    };
}

pub fn js_drop(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let result_opt = state.values.get(&id);
        if result_opt.is_none() {
            return Ok(cx.boolean(false));
        }
        state.values.insert(id, StateType::Freed);
        return Ok(cx.boolean(true));
    };
}
