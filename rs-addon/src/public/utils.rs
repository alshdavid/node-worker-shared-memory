use crate::state::{SyncState, ID, matches_number, matches_string, matches_vector};
use neon::prelude::*;

pub fn js_typeof(sync_state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsString> {
    return move |mut cx: FunctionContext| -> JsResult<JsString> {
        let state = sync_state.lock().unwrap();

        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let result_opt = state.values.get(&id);
        if result_opt.is_none() {
            return Ok(cx.string("undefined"));
        }

        let result = result_opt.unwrap();
        if matches_number(result).is_ok() {
            return Ok(cx.string("number"));
        }
        if matches_string(result).is_ok() {
            return Ok(cx.string("string"));
        }
        if matches_vector(result).is_ok() {
            return Ok(cx.string("vector"));
        }

        panic!("unknown type");
    };
}
