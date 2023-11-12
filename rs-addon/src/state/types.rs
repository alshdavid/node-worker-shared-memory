use std::sync::Arc;

#[derive(Clone)]
pub enum PrimitiveType {
    String(Arc<String>),
    Number(Arc<f64>),
}

#[derive(Clone)]
pub enum DynamicType {
    Vector(Arc<Vec<ID>>),
}

#[derive(Clone)]
pub enum StateType {
    PrimitiveType(PrimitiveType),
    DynamicType(DynamicType),
}

pub type ID = i128;

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

pub fn matches_vector(value: &StateType) -> Result<Arc<Vec<ID>>, ()> {
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
