use crate::state::{SyncState, ID};
use neon::prelude::*;

pub fn js_struct_factory_new(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let id = state.struct_factory_new();
        return Ok(cx.number(id as f64));
    };
}

pub fn js_struct_factory_define_key(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;
        let arg2: Handle<JsString> = cx.argument(2)?;

        let id = arg0.value(&mut cx).floor() as ID;
        let key_name = arg1.value(&mut cx);
        let value_type = arg2.value(&mut cx);

        let result = state.struct_factory_define_key(&id, key_name, value_type);

        return Ok(cx.boolean(result));
    };
}

pub fn js_struct_factory_instantiate(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;

        let id = arg0.value(&mut cx).floor() as ID;

        let result = state.struct_factory_instantiate(&id);

        return Ok(cx.number(result));
    };
}

pub fn js_struct_get_key(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;

        let id = arg0.value(&mut cx).floor() as ID;
        let key_name = arg1.value(&mut cx);

        let result = state.struct_get_key(&id, key_name);

        return Ok(cx.number(result));
    };
}

pub fn js_struct_set_key(state: SyncState) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(1)?;

        let id = arg0.value(&mut cx).floor() as ID;
        let key_name = arg1.value(&mut cx);

        let key_id = state.struct_get_key(&id, key_name);

        let type_of = state.type_of(&key_id);

        if type_of == "string" {
            let arg2: Handle<JsString> = cx.argument(2)?;
            let key_value = arg2.value(&mut cx);
            state.string_set(&key_id, key_value);
        } else if type_of == "number" {
            let arg2: Handle<JsNumber> = cx.argument(2)?;
            let key_value = arg2.value(&mut cx).floor();
            state.number_set(&key_id, key_value);
        } else {
            panic!("Unable to set struct type")
        }
        
        return Ok(cx.boolean(true));
    };
}