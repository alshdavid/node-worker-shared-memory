use std::sync::Arc;

use super::{PrimitiveType, State, StateType, ID, matches_string};

impl State {
    pub fn string_new(&mut self) -> ID {
        let value_arc = Arc::new(String::new());
        let id = self.new_id();
        self.values.insert(
            id,
            StateType::PrimitiveType(PrimitiveType::String(value_arc)),
        );
        return id;
    }

    pub fn string_set(&mut self, id: &ID, value: String) -> bool {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return false
        }
        let found_ref = found_opt.unwrap();
        let result = matches_string(found_ref);
        if result.is_err() {
            return false
        }
        self.values.insert(
            id.clone(),
            StateType::PrimitiveType(PrimitiveType::String(Arc::new(value))),
        );
        return true;
    }

    pub fn string_get(&mut self, id: &ID) -> Result<Arc<String>, ()> {
        let value_opt = self.values.get(id);
        if value_opt.is_none() {
            panic!("Value not available");
        }
        let found_ref = value_opt.unwrap();
        return matches_string(found_ref);        
    }

    pub fn string_delete(&mut self, id: &ID) -> bool {
        self.values.remove(id);
        return true;
    }
}
