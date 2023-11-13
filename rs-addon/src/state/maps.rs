use std::sync::Arc;

use dashmap::DashMap;

use super::{DynamicType, State, StateType, ID, matches_map};

impl State {
    pub fn map_new(&self) -> ID {
        let value_arc = Arc::new(DashMap::<String, ID>::new());
        let id = self.new_id();
        self.values
            .insert(id, StateType::DynamicType(DynamicType::Map(value_arc)));
        return id;
    }

    pub fn map_add(&self, id: &ID, key: String, insert: &ID) -> bool {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return false;
        }
        let found_ref = found_opt.unwrap();
        let result = matches_map(&found_ref);
        if result.is_err() {
            return false;
        }
        let map = result.unwrap();
        (*map).insert(key, insert.clone());
        return true;
    }

    pub fn map_get(&self, id: &ID, key: String) -> ID {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            todo!()
        }
        let found_ref = found_opt.unwrap();
        let result = matches_map(&found_ref);
        if result.is_err() {
            todo!()
        }
        let map = result.unwrap();
        let result = map.get(&key);
        if result.is_none() {
            todo!()
        }
        return result.unwrap().clone();
    }

    pub fn map_remove(&self, id: &ID, key: String) -> bool {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return false;
        }
        let found_ref = found_opt.unwrap();
        let result = matches_map(&found_ref);
        if result.is_err() {
            return false;
        }
        let map = result.unwrap();
        let result = map.remove(&key);
        if result.is_none() {
            return false;
        }
        return true;
    }

    pub fn map_len(&self, id: &ID) -> usize {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            todo!()
        }
        let found_ref = found_opt.unwrap();
        let result = matches_map(&found_ref);
        if result.is_err() {
            todo!()
        }
        let map = result.unwrap();
        return map.len();
    }
}
