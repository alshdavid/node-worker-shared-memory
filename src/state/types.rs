use std::{sync::{Arc, Mutex}, collections::HashMap};

use dashmap::DashMap;

#[derive(Clone)]
pub enum PrimitiveType {
    String(Arc<String>),
    Number(Arc<f64>),
    // Null,
}

#[derive(Clone)]
pub enum DynamicType {
    Vector(Arc<Mutex<Vec<ID>>>),
    Map(Arc<DashMap<String, ID>>),
    Struct(Arc<Mutex<HashMap<String, ID>>>),
}

#[derive(Clone)]
pub enum StateType {
    PrimitiveType(PrimitiveType),
    DynamicType(DynamicType),
    Freed,
}

pub type ID = u32;

pub fn matches_string(value: &StateType) -> Result<Arc<String>, ()> {
    return match value {
        StateType::PrimitiveType(p) => match p {
            PrimitiveType::String(s) => {
                return Ok(Arc::clone(&s));
            }
            _ => Err(()),
        },
        _ => Err(()),
    };
}

pub fn matches_number(value: &StateType) -> Result<Arc<f64>, ()> {
    return match value {
        StateType::PrimitiveType(p) => match p {
            PrimitiveType::Number(n) => {
                return Ok(Arc::clone(&n));
            }
            _ => Err(()),
        },
        _ => Err(()),
    };
}

pub fn matches_vector(value: &StateType) -> Result<Arc<Mutex<Vec<ID>>>, ()> {
    return match value {
        StateType::DynamicType(d) => match d {
            DynamicType::Vector(v) => {
                return Ok(Arc::clone(&v));
            }
            _ => Err(()),
        },
        _ => Err(()),
    };
}

pub fn matches_struct(value: &StateType) -> Result<Arc<Mutex<HashMap<String, ID>>>, ()> {
    return match value {
        StateType::DynamicType(d) => match d {
            DynamicType::Struct(s) => {
                return Ok(Arc::clone(&s));
            }
            _ => Err(()),
        },
        _ => Err(()),
    };
}

pub fn matches_map(value: &StateType) -> Result<Arc<DashMap<String, ID>>, ()> {
    return match value {
        StateType::DynamicType(d) => match d {
            DynamicType::Map(m) => {
                return Ok(Arc::clone(&m));
            }
            _ => Err(()),
        },
        _ => Err(()),
    };
}

pub fn matches_freed(value: &StateType) -> Result<bool, ()> {
    return match value {
        StateType::Freed => {
            return Ok(true);
        }
        _ => Err(()),
    };
}

// pub fn matches_null(value: &StateType) -> Result<bool, ()> {
//     return match value {
//         StateType::PrimitiveType(p) => match p {
//             PrimitiveType::Null => {
//                 return Ok(true);
//             }
//             _ => Err(()),
//         },
//         _ => Err(()),
//     };
// }
