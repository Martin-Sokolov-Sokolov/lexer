use std::{cell::RefCell, collections::HashMap, rc::Rc};


use crate::{evaluator::{RuntimeError, RuntimeException}, expr::Literal, token::Token};

#[derive(Clone)]
pub struct Environment {
    values: HashMap<String, Option<Box<Literal>>>,
    enclosing: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn define (&mut self, name: String, b: Option<Box<Literal>>) {
        self.values.insert(name, b);
    }

    pub fn get(&self, name: &Token) -> Result<Option<Box<Literal>>, RuntimeException> {
        if let Some(val) = self.values.get(&name.lexeme) {
            return Ok(val.clone());
        }

        if let Some(helper) = &self.enclosing {
            return helper.borrow().get(name).clone();
        }

        return Err(RuntimeException::RuntimeError(RuntimeError::new(name, format!("Undefined variable '{}'", name.lexeme).as_str())));    }

    pub fn assign(&mut self, name: &Token, value: Option<&Box<Literal>>) -> Result<(), RuntimeException>{

        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.cloned());
            return Ok(());
        }

        if let Some(helper) = &self.enclosing {
            helper.borrow_mut().assign(&name, value)?;
            return Ok(());
        }

        return Err(RuntimeException::RuntimeError(RuntimeError::new(name, format!("Undefined variable '{}'.", name.lexeme).as_str())));
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