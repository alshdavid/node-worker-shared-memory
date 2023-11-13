mod state;
mod public;

use neon::prelude::*;
use once_cell::sync::Lazy;
use state::{SyncState, State};

static STATE: Lazy<SyncState> = Lazy::new(|| { return State::new_sync() });

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let state = STATE.clone();

    // Handle strings
    cx.export_function("js_string_new", public::js_string_new(state.clone())).unwrap();
    cx.export_function("js_string_get", public::js_string_get(state.clone())).unwrap();
    cx.export_function("js_string_set", public::js_string_set(state.clone())).unwrap();
    cx.export_function("js_string_delete", public::js_string_delete(state.clone())).unwrap();

    // Handle numbers
    cx.export_function("js_number_new", public::js_number_new(state.clone())).unwrap();
    cx.export_function("js_number_get", public::js_number_get(state.clone())).unwrap();
    cx.export_function("js_number_set", public::js_number_set(state.clone())).unwrap();
    cx.export_function("js_number_delete", public::js_number_delete(state.clone())).unwrap();

    cx.export_function("js_vectors_new", public::js_vectors_new(state.clone())).unwrap();
    cx.export_function("js_vectors_push", public::js_vectors_push(state.clone())).unwrap();
    cx.export_function("js_vectors_index", public::js_vectors_index(state.clone())).unwrap();
    cx.export_function("js_vectors_len", public::js_vectors_len(state.clone())).unwrap();

    cx.export_function("js_typeof", public::js_typeof(state.clone())).unwrap();

    Ok(())
}
