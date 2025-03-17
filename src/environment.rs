use std::collections::HashMap;

use crate::expr::Literal;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Box<Literal>>>,
    enclosing: Box<Option<Environment>>,
}

impl Environment {
    pub fn define (&mut self, name: String, b: Option<Box<Literal>>) {
        self.values.insert(name, b);
    }

    pub fn get(&self, name: &String) -> Result<&Option<Box<Literal>>, String> {
        if self.values.contains_key(name) {
            return Ok(self.values.get(name).unwrap());
        }

        if let Some(helper) = &*self.enclosing {
            return helper.get(name);
        }

        Err("Undefined variable.".to_string())
    }

    pub fn assign(&mut self, s: &String, a: Option<&Box<Literal>>) -> Result<(), String>{
        if self.values.contains_key(s) {
            self.values.insert(s.to_string(), a.cloned());
            return Ok(());
        }

        if let Some(helper) = self.enclosing.as_mut() {
            helper.assign(s, a)?;
            return Ok(());
        }

        return Err(format!("Undefined variable '{}'.", s));

    }

    pub fn new_enclosing(env: &Environment) -> Self {
        Self {
            values: env.clone().values,
            enclosing: Box::from(None),
        }
    }

    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Box::from(None),
        }
    }

}