use std::sync::Arc;

use super::{matches_number, PrimitiveType, State, StateType, ID};

impl State {
    pub fn number_new(&self) -> ID {
        let value_arc = Arc::new(0.0);
        let id = self.new_id();
        self.values.insert(
            id,
            StateType::PrimitiveType(PrimitiveType::Number(value_arc)),
        );
        return id;
    }

    pub fn number_set(&self, id: &ID, value: f64) -> bool {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            return false;
        }
        let found_ref = found_opt.unwrap();
        let result = matches_number(&found_ref);
        if result.is_err() {
            return false;
        }
        self.values.insert(
            id.clone(),
            StateType::PrimitiveType(PrimitiveType::Number(Arc::new(value))),
        );
        return true;
    }

    pub fn number_get(&self, id: &ID) -> Result<Arc<f64>, ()> {
        let value_opt = self.values.get(id);
        if value_opt.is_none() {
            return Err(());
        }
        let found_ref = value_opt.unwrap();
        return matches_number(&found_ref);        
    }

    pub fn number_delete(&self, id: &ID) -> bool {
        self.values.remove(id);
        return true;
    }
}
