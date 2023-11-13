use crate::state::{SyncState, ID};
use neon::prelude::*;

pub fn js_maps_new(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let id = state.map_new();
        return Ok(cx.number(id as f64));
    };
}

pub fn js_maps_add(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;
        let arg2: Handle<JsNumber> = cx.argument(2)?;
        
        let id = arg0.value(&mut cx).floor() as ID;
        let key = arg1.value(&mut cx) as String;
        let insert = arg2.value(&mut cx).floor() as ID;

        let result = state.map_add(&id, key, &insert);
        return Ok(cx.boolean(result));
    };
}

pub fn js_maps_get(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;
        
        let id = arg0.value(&mut cx).floor() as ID;
        let key = arg1.value(&mut cx) as String;

        let result = state.map_get(&id, key);
        return Ok(cx.number(result));
    };
}

pub fn js_maps_remove(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;
        
        let id = arg0.value(&mut cx).floor() as ID;
        let key = arg1.value(&mut cx) as String;

        let result = state.map_remove(&id, key);
        return Ok(cx.boolean(result));
    };
}

pub fn js_maps_len(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        
        let id = arg0.value(&mut cx).floor() as ID;

        let result = state.map_len(&id);
        return Ok(cx.number(result as f64));
    };
}
