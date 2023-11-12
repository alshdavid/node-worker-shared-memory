use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use dashmap::DashMap;

use crate::state::{matches_number, matches_string, matches_vector};

use super::{StateType, ID, matches_freed, matches_struct};

pub type SyncState = Arc<State>;

#[derive(Clone)]
pub struct State {
    pub counter: Arc<Mutex<ID>>,
    pub values: Arc<DashMap<ID, StateType>>,
    pub struct_definitions: Arc<Mutex<HashMap<ID, Arc<Mutex<HashMap<String, String>>>>>>,
}

impl State {
    pub fn new() -> Self {
        return State {
            counter: Arc::new(Mutex::new(0)),
            values: Arc::new(DashMap::new()),
            struct_definitions: Arc::new(Mutex::new(HashMap::new())),
        };
    }

    pub fn new_sync() -> SyncState {
        return Arc::new(State::new());
    }

    pub fn free_value(&self, id: &ID) -> bool {
        let result_opt = self.values.get(&id);
        if result_opt.is_none() {
            return false;
        }
        let value_at = result_opt.unwrap();
        if matches_struct(&value_at).is_ok() {
            let keys = self.struct_get_keys(id);
            for (_, value_id) in keys.iter() {
                self.values.insert(value_id.clone(), StateType::Freed);
            }
        }
        self.values.insert(id.clone(), StateType::Freed);
        return true;
    }

    pub fn new_id(&self) -> ID {
        let mut counter = self.counter.lock().unwrap();
        let id = counter.clone();
        (*counter) += 1;
        return id;
    }

    // pub fn get_value(&self, id: &ID) -> Option<StateType> {
    //     let found_opt = self.values.get(id);
    //     if found_opt.is_none() {
    //         todo!()
    //     }
    //     let found_ref = found_opt.unwrap();
    //     return Some(found_ref.clone());
    // }

    // pub fn set_value_existing(&self, id: &ID, value: StateType) -> bool {
    //     self.values.insert(id.clone(), value);
    //     return true;
    // }

    pub fn set_value_new(&self, value: StateType) -> ID {
        let id = self.new_id();
        self.values.insert(id.clone(), value);
        return id;
    }

    pub fn type_of(&self, id: &ID) -> String {
        let result_opt = self.values.get(&id);
        if result_opt.is_none() {
            return String::from("undefined");
        }

        let result = result_opt.unwrap();
        if matches_number(&result).is_ok() {
            return String::from("number");
        }
        if matches_string(&result).is_ok() {
            return String::from("string");
        }
        if matches_vector(&result).is_ok() {
            return String::from("vector");
        }
        if matches_freed(&result).is_ok() {
            return String::from("freed");
        }
        if matches_struct(&result).is_ok() {
            return String::from("struct");
        }
        // if matches_null(&result).is_ok() {
        //     return String::from("null");
        // }

        return String::from("unknown type");
    }
}
