use std::any::Any;

use crate::parser::{Expr, Literal, UnaryOp};

pub struct Evaluator;

trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>>;
}

trait Visitor {
    fn visit_literal(&self, lit: &Literal) -> Option<Box<dyn Any>>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Option<Box<dyn Any>>;
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Option<Box<dyn Any>>;
}

impl Visitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Option<Box<dyn Any>> {
        match expr {
            Literal::Nil => Some(Box::new("nil".to_string())),
            Literal::False(b) => Some(Box::from(*b)),
            Literal::True(b) => Some(Box::from(*b)),
            Literal::Number(n) => Some(Box::from(*n)),
            Literal::Str(str) => Some(Box::new(str.clone())),
        }
    }

    fn visit_grouping(&mut self, box_expr: &Box<Expr>) -> Option<Box<dyn Any>> {
        self.evaluate(&box_expr)
    }
    
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Option<Box<dyn Any>> {
        let _r = self.evaluate(&un)?;
        match op {
            UnaryOp::Negate => {
                let num = _r.downcast_ref::<f64>()?;
                return Some(Box::new(-num));
            },
            UnaryOp::Not => {
                return Some(Box::new(!self.is_truthy(&_r)));
            }
        }
    }
}

impl Accept for Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            Expr::Grouping(gr) => visitor.visit_grouping(gr),
            Expr::Unary(op, b) => visitor.visit_unary(op, b),
            _ => None
        }
    }
}

impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) -> Option<Box<dyn Any>> {
        expr.accept(self)
    }

    pub fn is_truthy(&self, r: &Box<dyn Any>) -> bool {
        let _op_bool = r.downcast_ref::<bool>();

        match _op_bool {
            Some(b) => return *b,
            None => false,
        }
    }

    pub fn writer(&self, value: &Box<dyn Any>) {
        if let Some(val) = value.downcast_ref::<f64>() {
            println!("{}", val);
        }
        else if let Some(val) = value.downcast_ref::<String>() {
            println!("{}", val);
        }
        else if let Some(val) = value.downcast_ref::<bool>() {
            println!("{}", val);
        }
        else {
            println!("not implemented");
        }
    }
}