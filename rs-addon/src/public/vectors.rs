use crate::state::{SyncState, ID};
use neon::prelude::*;

pub fn js_vectors_new(sync_state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let mut state = sync_state.lock().unwrap();
        let id = state.vector_new();
        return Ok(cx.number(id as f64));
    };
}

pub fn js_vectors_push(sync_state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let mut state = sync_state.lock().unwrap();

        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsNumber> = cx.argument(1)?;
        
        let id = arg0.value(&mut cx).floor() as ID;
        let insert = arg1.value(&mut cx).floor() as ID;

        let result = state.vector_push(&id, &insert);
        return Ok(cx.boolean(result));
    };
}

pub fn js_vectors_index(sync_state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let mut state = sync_state.lock().unwrap();

        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsNumber> = cx.argument(1)?;
        
        let id = arg0.value(&mut cx).floor() as ID;
        let index = arg1.value(&mut cx).floor() as usize;

        let result = state.vector_index(&id, index);
        if result.is_err() {
            panic!("Failed");
        }
        return Ok(cx.number(result.unwrap()));
    };
}

pub fn js_vectors_len(sync_state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let mut state = sync_state.lock().unwrap();

        let arg0: Handle<JsNumber> = cx.argument(0)?;
        
        let id = arg0.value(&mut cx).floor() as ID;

        let result = state.vector_len(&id);
        if result.is_err() {
            panic!("Failed");
        }
        return Ok(cx.number(result.unwrap() as f64));
    };
}
