use std::sync::Arc;

use super::{matches_vector, DynamicType, State, StateType, ID};

impl State {
    pub fn vector_new(&mut self) -> ID {
        let value_arc = Arc::new(Vec::<ID>::new());
        let id = self.new_id();
        self.values
            .insert(id, StateType::DynamicType(DynamicType::Vector(value_arc)));
        return id;
    }

    pub fn vector_push(&mut self, id: &ID, value: StateType) -> Result<ID, ()> {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            panic!("Value not available");
        }
        let found_ref = found_opt.unwrap();
        let result = matches_vector(found_ref);
        if result.is_err() {
            return Err(());
        }
        todo!();
    }
}
