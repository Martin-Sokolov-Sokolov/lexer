use crate::{evaluator::Evaluator, expr::Literal};

pub trait LoxCallable {
    fn callq(&self, evaluator: &mut Evaluator, arguments: Vec<Literal>) -> Result<Option<Box<Literal>>, String>;
    fn arrity(&self) -> usize;
    fn to_string(&self) -> String;
}