use crate::{evaluator::{Evaluator, RuntimeException}, expr::Literal};

pub trait LoxCallable {
    fn callq(&self, evaluator: &mut Evaluator, arguments: Vec<Literal>) -> Result<Option<Box<Literal>>, RuntimeException>;
    fn arrity(&self) -> usize;
    fn to_string(&self) -> String;
}