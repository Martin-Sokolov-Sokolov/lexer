use crate::{evaluator::Evaluator, expr::{Expr, Literal}};

pub trait LoxCallable {
    fn callq(&self, evaluator: &mut Evaluator, arguments: Vec<Expr>) -> Box<Literal>;
    fn arrity(&self) -> usize;
    fn to_string(&self) -> String;
}
