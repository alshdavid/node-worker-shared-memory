use std::sync::{Arc, Mutex};

use dashmap::DashMap;

use super::{StateType, ID};


pub type SyncState = Arc<State>;

#[derive(Clone)]
pub struct State {
    pub counter: Arc<Mutex<ID>>,
    pub values: Arc<DashMap<ID, StateType>>,
}

impl State {
    pub fn new() -> Self {
        return State{
            counter: Arc::new(Mutex::new(0)),
            values: Arc::new(DashMap::new()),
        };
    }

    pub fn new_sync() -> SyncState {
        return Arc::new(State::new());
    }

    pub fn new_id(&self) -> ID {
        let mut counter = self.counter.lock().unwrap();
        let id = counter.clone();
        (*counter) += 1;
        return  id;
    }
}
