use std::{sync::{Arc, Mutex}, collections::HashMap};

use super::{State, ID, DynamicType, StateType, matches_struct};

impl State {
    pub fn struct_factory_new(&self) -> ID {
        let id = self.new_id();
        let insert = Arc::new(Mutex::new(HashMap::<String, String>::new()));
        self.struct_definitions.lock().unwrap().insert(
            id.clone(),
            insert,
        );
        return id.clone();
    }

    pub fn struct_factory_define_key(&self, id: &ID, key_name: String, value_type: String) -> bool {
        let mut found_opt = self.struct_definitions.lock().unwrap();
        let result_opt = found_opt.get_mut(&id);
        if result_opt.is_none() {
            return false;
        }
        let found_mutex = result_opt.unwrap();
        let mut found_ref = found_mutex.lock().unwrap();
        (*found_ref).insert(key_name, value_type);
        return true;
    }

    pub fn struct_factory_instantiate(&self, id: &ID) -> ID {
        let mut found_opt = self.struct_definitions.lock().unwrap();
        let result_opt = found_opt.get_mut(&id);
        if result_opt.is_none() {
            todo!();
        }
        let found_mutex = result_opt.unwrap();
        let found_ref = found_mutex.lock().unwrap();
        let mut struct_resources = HashMap::<String, ID>::new();
        for (key, value) in found_ref.iter() {
            if value == "string" {
                let id = self.string_new();
                struct_resources.insert(key.clone(), id);
            }
            if value == "number" {
                let id = self.number_new();
                struct_resources.insert(key.clone(), id);
            }
        }
        let insert = Arc::new(Mutex::new(struct_resources));
        let id = self.new_id();
        self.values
            .insert(id, StateType::DynamicType(DynamicType::Struct(insert)));
        return id;
    }

    pub fn struct_get_keys(&self, id: &ID) -> HashMap<String, ID> {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            todo!()
        }
        let found_ref = found_opt.unwrap();
        let result = matches_struct(&found_ref);
        if result.is_err() {
            todo!()
        }
        let map = result.unwrap();
        let struct_fields = map.lock().unwrap();
        return struct_fields.clone();        
    }

    pub fn struct_get_key(&self, id: &ID, key_name: String) -> ID {
        let found_opt = self.values.get_mut(id);
        if found_opt.is_none() {
            todo!()
        }
        let found_ref = found_opt.unwrap();
        let result = matches_struct(&found_ref);
        if result.is_err() {
            todo!()
        }
        let map = result.unwrap();
        let struct_fields = map.lock().unwrap();
        let found = struct_fields.get(&key_name);
        if found.is_none() {
            todo!()
        }
        let key_id = found.unwrap();
        return key_id.clone();
    }

}
