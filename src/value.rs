use std::collections::HashMap;
use variable::Variable;

#[derive(Clone, PartialEq)]
pub enum Value {
  Undefined,
  Variable(Variable),
  Object(HashMap<String, Value>),
  String(String),
  Array(Vec<Value>),
  Number(u32),
  Boolean(bool),
}
