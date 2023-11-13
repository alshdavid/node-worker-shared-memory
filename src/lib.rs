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

    // Handle numbers
    cx.export_function("js_number_new", public::js_number_new(state.clone())).unwrap();
    cx.export_function("js_number_get", public::js_number_get(state.clone())).unwrap();
    cx.export_function("js_number_set", public::js_number_set(state.clone())).unwrap();

    cx.export_function("js_vectors_new", public::js_vectors_new(state.clone())).unwrap();
    cx.export_function("js_vectors_push", public::js_vectors_push(state.clone())).unwrap();
    cx.export_function("js_vectors_index", public::js_vectors_index(state.clone())).unwrap();
    cx.export_function("js_vectors_len", public::js_vectors_len(state.clone())).unwrap();

    cx.export_function("js_maps_new", public::js_maps_new(state.clone())).unwrap();
    cx.export_function("js_maps_add", public::js_maps_add(state.clone())).unwrap();
    cx.export_function("js_maps_get", public::js_maps_get(state.clone())).unwrap();
    cx.export_function("js_maps_remove", public::js_maps_remove(state.clone())).unwrap();
    cx.export_function("js_maps_len", public::js_maps_len(state.clone())).unwrap();

    cx.export_function("js_typeof", public::js_typeof(state.clone())).unwrap();
    cx.export_function("js_drop", public::js_drop(state.clone())).unwrap();


    cx.export_function("js_struct_factory_new", public::js_struct_factory_new(state.clone())).unwrap();
    cx.export_function("js_struct_factory_define_key", public::js_struct_factory_define_key(state.clone())).unwrap();
    cx.export_function("js_struct_factory_instantiate", public::js_struct_factory_instantiate(state.clone())).unwrap();
    cx.export_function("js_struct_get_key", public::js_struct_get_key(state.clone())).unwrap();
    cx.export_function("js_struct_set_key", public::js_struct_set_key(state.clone())).unwrap();

    Ok(())
}
