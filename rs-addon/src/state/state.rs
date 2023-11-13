use std::{sync::{Arc, Mutex}, collections::HashMap};

use super::{StateType, ID};


pub type SyncState = Arc<Mutex<State>>;

pub struct State {
    pub counter: ID,
    pub values: HashMap<ID, StateType>,
}

impl State {
    pub fn new() -> Self {
        return State{
            counter: 0,
            values: HashMap::new(),
        };
    }

    pub fn new_sync() -> SyncState {
        return Arc::new(Mutex::new(State::new()));
    }

    pub fn new_id(&mut self) -> ID {
        let id = self.counter;
        self.counter += 1;
        return  id;
    }
}
