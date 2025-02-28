use std::any::Any;

use crate::parser::*;

pub struct Evaluator;

trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>>;
}

trait Visitor {
    fn visit_literal(&self, lit: &Literal) -> Option<Box<dyn Any>>;
}

impl Visitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Option<Box<dyn Any>>{
        match expr {
            Literal::Nil => None,
            Literal::False(b) => Some(Box::from(*b)),
            Literal::True(b) => Some(Box::from(*b)),
            Literal::Number(n) => Some(Box::from(*n)),
            Literal::Str(str) => Some(Box::new(str.clone())),
            _ => None
        }
    }
}

impl Accept for Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            _ => None
        }
    }
}

impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) {
        if let Some(res) = expr.accept(self) {
            println!("{:?}", res);
        }
    }
}