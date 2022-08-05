use std::{rc::Rc, cell::RefCell};

use crate::ast::{Context, codegen::type_inference::TypeInferenceMap};

#[derive(Debug)]
pub enum Type {
  String,
  Name,
  Bool,
  Int,
  Float,
  Identifier(String),
  Void,
  Unknown
}

impl ToString for Type {
    fn to_string(&self) -> String {
      match self {
        Type::String => "string".to_string(),
        Type::Name => "name".to_string(),
        Type::Bool => "bool".to_string(),
        Type::Int => "int".to_string(),
        Type::Float => "float".to_string(),
        Type::Identifier(s) => s.clone(),
        Type::Void => "void".to_string(),
        Type::Unknown => "unknown".to_string(),
    }
  }
}

pub trait ToType {
  fn resulting_type(
    &self,
    _: &Rc<RefCell<Context>>,
    _: &TypeInferenceMap
  ) -> Type {
    Type::Unknown
  }
}