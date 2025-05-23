use std::{cell::{Ref, RefCell}, rc::Rc};

use crate::{environment::Environment, evaluator::{Evaluator, RuntimeException}, expr::{Literal, LoxCallables}, lox_callable::LoxCallable, stmt::FunctionStmt};

#[derive(PartialEq, Debug, Clone)]
pub struct LoxFunction {
    pub declaration: FunctionStmt,
    pub closure: Rc<RefCell<Environment>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct LoxAnonymous {
    callq: fn(
        &mut Evaluator,
        Vec<Literal>,
    ) -> Result<Option<Box<Literal>>, RuntimeException>,
    arrity: fn() -> usize,
}

impl LoxFunction {
    pub fn new(declaration: FunctionStmt, closure: Rc<RefCell<Environment>>) -> Self {
        LoxFunction { declaration, closure }
    }
}

impl LoxCallable for LoxCallables {
    fn callq(&self, evaluator: &mut Evaluator, arguments: Vec<Literal>) -> Result<Option<Box<Literal>>, RuntimeException> {
        match self {
            LoxCallables::LoxFunction(lc) => lc.callq(evaluator, arguments),
            LoxCallables::LoxAnonymous(la) => (la.callq)(evaluator, arguments),
        }
    }

    fn arrity(&self) -> usize {
        match self {
            LoxCallables::LoxFunction(lc) => lc.arrity(),
            LoxCallables::LoxAnonymous(la) => (la.arrity)(),
        }
    }

    fn to_string(&self) -> String {
        todo!()
    }
}

impl LoxCallable for LoxFunction {
    fn callq(&self, evaluator: &mut Evaluator, arguments: Vec<Literal>) -> Result<Option<Box<Literal>>, RuntimeException> {
        let mut env = Environment::new(Some(evaluator.globals.clone()));
        for i in 0..self.declaration.params.len() {
            env.define(self.declaration.params[i].clone().lexeme, Some(Box::from(arguments[i].clone())));
        }

        return evaluator.execute_block(&self.declaration.body, Rc::from(RefCell::from(env)))
                        .map(|_| None);
    }

    fn arrity(&self) -> usize {
        self.declaration.params.len()
    }

    fn to_string(&self) -> String {
        return format!("<fn {} >", self.declaration.name.lexeme);
    }
}

impl LoxAnonymous {
    pub fn new(
        callq: fn(
            &mut Evaluator,
            Vec<Literal>,
        ) -> Result<Option<Box<Literal>>, RuntimeException>,
        arrity: fn() -> usize,
    ) -> LoxAnonymous {
        LoxAnonymous {
            callq,
            arrity,
        }
    }
}