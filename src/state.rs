use std::collections::HashMap;
use variable::Variable;
use value::Value;

#[derive(Clone)]
pub struct State {
  variables: Vec<Variable>,
  values: HashMap<String, Value>,
}

impl State {
  fn create_variables(&self, names: Vec<String>) -> (State, Vec<Variable>) {
    let new_vars: Vec<Variable> = names.iter().map(|name| Variable { name: name.clone() }).collect();   
    let mut merge_vars: Vec<Variable> = self.variables.clone();
    merge_vars.extend(new_vars.clone().into_iter());
    (State { variables: merge_vars, values: self.values.clone() },
      new_vars)
  }

  fn assign_values(&self, new_vals: HashMap<String, Value>) -> State {
    let mut vals = self.values.clone();
    vals.extend(new_vals);
    State { variables: self.variables.clone(), values: vals }
  }
  
  fn value_of(&self, variable: &Variable) -> Value {
    let name = &variable.name;
    match self.values.get(name) {
      Some(value) => match value {
        &Value::Variable(ref value) => self.value_of(value),
        value => value.clone(),
      },
      None         => Value::Undefined,
    }
  }

  fn unify(&self, a: &Variable, b: &Variable) -> Option<State> {
    let a = self.value_of(a);
    let b = self.value_of(b);
    let mut new_val = HashMap::new();

    if a == b {
      return Some(self.clone());   
    } 

    match a {
      Value::Variable(ref a) => {
        new_val.insert(a.name.clone(), b);
        return Some(self.assign_values(new_val));
      },
      _ => (),
    }
    
    match b {
      Value::Variable(ref b) => {
        new_val.insert(b.name.clone(), a);
        return Some(self.assign_values(new_val));
      },
      _ => ()
    }
    
    None
  }

}


