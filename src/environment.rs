use std::{cell::RefCell, collections::HashMap, rc::Rc};


use crate::expr::Literal;

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Box<Literal>>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn define (&mut self, name: String, b: Option<Box<Literal>>) {
        self.values.insert(name, b);
    }

    pub fn get(&self, name: &String) -> Result<Option<Box<Literal>>, String> {
        if let Some(val) = self.values.get(name) {
            return Ok(val.clone());
        }

        if let Some(helper) = &self.enclosing {
            return helper.borrow().get(name).clone();
        }

        Err("Undefined variable.".to_string())
    }

    pub fn assign(&mut self, s: &String, a: Option<&Box<Literal>>) -> Result<(), String>{
        if self.values.contains_key(s) {
            self.values.insert(s.to_string(), a.cloned());
            return Ok(());
        }

        if let Some(helper) = &self.enclosing {
            helper.borrow_mut().assign(s,a)?;
            return Ok(());
        }

        return Err(format!("Undefined variable '{}'.", s));

    }

    pub fn new_enclosing(env: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            enclosing: Some(env),
        }
    }

    pub fn new() -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

}