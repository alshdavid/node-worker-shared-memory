mod state;

use std::{collections::HashMap, rc::Rc, cell::RefCell};

use crate::state::{string_get, string_new};
use neon::prelude::*;
use state::{ID, string_set, PrimitiveType, State};

fn js_string_new(state: Rc<RefCell<State>>) -> impl Fn(FunctionContext) -> JsResult<JsNumber> {
    return move |mut cx: FunctionContext| -> JsResult<JsNumber> {
        let id = string_new(state.clone());
        return Ok(cx.number(id as f64));
    };
}

fn js_string_get(state: Rc<RefCell<State>>) -> impl Fn(FunctionContext) -> JsResult<JsString> {
    return move |mut cx: FunctionContext| -> JsResult<JsString> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let id = arg0.value(&mut cx).floor() as ID;

        let v = string_get(state.clone(), &id);
        return Ok(cx.string(v.as_str()));
    }
}

fn js_string_set(state: Rc<RefCell<State>>) -> impl Fn(FunctionContext) -> JsResult<JsBoolean> {
    return move |mut cx: FunctionContext| -> JsResult<JsBoolean> {
        let arg0: Handle<JsNumber> = cx.argument(0)?;
        let arg1: Handle<JsString> = cx.argument(0)?;

        let id = arg0.value(&mut cx).floor() as ID;
        let update = arg1.value(&mut cx);

        let ok = string_set(state.clone(), &id, update);
        return Ok(cx.boolean(ok));
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let state = Rc::new(RefCell::new(HashMap::<ID, PrimitiveType>::new()));

    cx.export_function("js_string_new", js_string_new(state.clone())).unwrap();
    cx.export_function("js_string_get", js_string_get(state.clone())).unwrap();
    cx.export_function("js_string_set", js_string_set(state.clone())).unwrap();
    Ok(())
}