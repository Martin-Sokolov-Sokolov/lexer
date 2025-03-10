use std::{any::Any, collections::HashMap};

pub struct Environment {
    values: HashMap<String, Option<Box<dyn Any>>>,
}

impl Environment {
    pub fn define (&mut self, name: String, b: Option<Box<dyn Any>>) {
        self.values.insert(name, b);
    }

    pub fn get(&self, name: &String) -> Result<&Option<Box<dyn Any>>, String> {
        if self.values.contains_key(name) {
            return  Ok(self.values.get(name).unwrap());
        }
        Err("Undefined variable.".to_string())
    }

    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
        }
    }
}