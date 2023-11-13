use std::{sync::{Arc, Mutex}, ops::Index};

use super::{matches_vector, DynamicType, State, StateType, ID};

impl State {
    pub fn vector_new(&self) -> ID {
        let value_arc = Arc::new(Mutex::new(Vec::<ID>::new()));
        let id = self.new_id();
        self.values
            .insert(id, StateType::DynamicType(DynamicType::Vector(value_arc)));
        return id;
    }

    pub fn vector_push(&self, id: &ID, insert: &ID) -> bool {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return false;
        }
        let found_ref = found_opt.unwrap();
        let result = matches_vector(&found_ref);
        if result.is_err() {
            return false;
        }
        let vec_mutex = result.unwrap();
        let mut vec = vec_mutex.lock().unwrap();
        (*vec).push(insert.clone());
        return true;
    }

    pub fn vector_index(&self, id: &ID, index: usize) -> Result<ID, ()> {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return Err(());
        }
        let found_ref = found_opt.unwrap();
        let result = matches_vector(&found_ref);
        if result.is_err() {
            return Err(());
        }
        let vec_mutex = result.unwrap();
        let vec = vec_mutex.lock().unwrap();
        if vec.len() < index {
            return Err(());
        }
        let found = vec.index(index);
        return Ok(found.clone());
    }

    pub fn vector_len(&self, id: &ID) -> Result<usize, ()> {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return Err(());
        }
        let found_ref = found_opt.unwrap();
        let result = matches_vector(&found_ref);
        if result.is_err() {
            return Err(());
        }
        let vec_mutex = result.unwrap();
        let vec = vec_mutex.lock().unwrap();
        return Ok(vec.len());
    }
}
