use std::{collections::HashMap, rc::Rc, cell::RefCell};

#[derive(Clone)]
pub enum PrimitiveType {
    String(Rc<String>),
    // Float(Rc<f64>),
    // Int(Rc<i64>),
    // Undefined,
}

pub type ID = usize;
pub type State = HashMap<ID, PrimitiveType>;

pub fn string_new(state_cell: Rc<RefCell<State>>) -> ID {
    let mut state = state_cell.borrow_mut();
    let id = state.len();
    let insert = Rc::new(String::new());
    state.insert(id, PrimitiveType::String(insert));
    return id;
}

pub fn string_set(state_cell: Rc<RefCell<State>>, id: &ID, value: String) -> bool {
  let mut state = state_cell.borrow_mut();
  let found_opt = state.get_mut(id);
    if found_opt.is_none() {
        panic!("Value not available");
    }
    let found_ref = found_opt.unwrap();
    match found_ref {
        PrimitiveType::String(_v) => {
          state.insert(id.clone(), PrimitiveType::String(Rc::new(value)));
          return true;
        },
        
        // PrimitiveType::Float(_v) => {
        //     panic!("Value not available");
        // },
        // PrimitiveType::Int(_v) => {
        //     panic!("Value not available");
        // },
        // PrimitiveType::Undefined => {
        //     panic!("Value not available");
        // },
    }
}

pub fn string_get(state_cell: Rc<RefCell<State>>, id: &ID) -> Rc<String> {
    let state = state_cell.borrow();
    let value_opt = state.get(id);
    if value_opt.is_none() {
        panic!("Value not available");
    }
    let value_ref = value_opt.unwrap();
    match value_ref {
        PrimitiveType::String(v) => {
            return  Rc::clone(v);
        },
        // PrimitiveType::Float(_v) => {
        //     panic!("Value not available");
        // },
        // PrimitiveType::Int(_v) => {
        //     panic!("Value not available");
        // },
        // PrimitiveType::Undefined => {
        //     panic!("Value not available");
        // },
    }
}
