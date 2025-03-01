use std::{any::Any, process};
use crate::parser::{BinaryOp, Expr, Literal, UnaryOp};

pub struct Evaluator;

trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>>;
}

trait Visitor {
    fn visit_literal(&self, lit: &Literal) -> Option<Box<dyn Any>>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Option<Box<dyn Any>>;
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Option<Box<dyn Any>>;
    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Option<Box<dyn Any>>;
}

impl Visitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Option<Box<dyn Any>> {
        match expr {
            Literal::Nil => Some(Box::new(Literal::Nil)),
            Literal::Boolean(b) => Some(Box::from(*b)),
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

    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Option<Box<dyn Any>> {
        let _op_left = self.evaluate(&left)?;
        let _op_right = self.evaluate(&right)?;

        match op {
            BinaryOp::Add => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l + r))
                }
                let (vl, vr) = (_op_left.downcast_ref::<String>(), _op_right.downcast_ref::<String>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    let mut res = String::from(l);
                    res.push_str(r);
                    return Some(Box::new(res));
                }
                None
            }
            BinaryOp::Subtract => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l - r))
                }
                None
            }
            BinaryOp::Multiply => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l * r))
                }
                None
            }
            BinaryOp::Divide => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    if *r == 0.0 {
                        process::exit(65);
                    }
                    return Some(Box::new(l / r))
                }
                None
            }
            BinaryOp::Greater => {
                let (vl, vr) = (_op_left.downcast_ref::<bool>(), _op_right.downcast_ref::<bool>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l > r))
                }
                None
            }
            BinaryOp::GreaterEqual => {
                let (vl, vr) = (_op_left.downcast_ref::<bool>(), _op_right.downcast_ref::<bool>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l >= r))
                }
                None
            }
            BinaryOp::Less => {
                let (vl, vr) = (_op_left.downcast_ref::<bool>(), _op_right.downcast_ref::<bool>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l < r))
                }
                None
            }
            BinaryOp::LessEqual => {
                let (vl, vr) = (_op_left.downcast_ref::<bool>(), _op_right.downcast_ref::<bool>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Some(Box::new(l <= r))
                }
                None
            }
            BinaryOp::EqualEqual => {
                return Some(Box::new(self.is_equal(&_op_left, &_op_right)));
            }
            BinaryOp::NotEquals => {
                return Some(Box::new(!self.is_equal(&_op_left, &_op_right)));
            }
            _ => return None,
        }

    }

}

impl Accept for Expr {
    fn accept(&self, visitor: &mut dyn Visitor) -> Option<Box<dyn Any>> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            Expr::Grouping(gr) => visitor.visit_grouping(gr),
            Expr::Unary(op, b) => visitor.visit_unary(op, b),
            Expr::Binary(left, op, right) => visitor.visit_binary(op, left, right),
        }
    }
}

impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) -> Option<Box<dyn Any>> {
        expr.accept(self)
    }

    fn is_equal(&self, a: &Box<dyn Any>, b: &Box<dyn Any>) -> bool {
        if a.type_id() != b.type_id() {
            return false;
        }
    
        if let (Some(a_val), Some(b_val)) = (a.downcast_ref::<String>(), b.downcast_ref::<String>()) {
            return a_val == b_val;
        }
        else if let (Some(a_val), Some(b_val)) = (a.downcast_ref::<f64>(), b.downcast_ref::<f64>()) {
            return a_val == b_val;
        }
        else if let (Some(a_val), Some(b_val)) = (a.downcast_ref::<bool>(), b.downcast_ref::<bool>()) {
            return a_val == b_val;
        }
    
        false
    }

    pub fn is_truthy(&self, r: &Box<dyn Any>) -> bool {
        if r.is::<Literal>() {
            if let Some(nil_val) = r.downcast_ref::<Literal>() {
                match nil_val {
                    Literal::Nil => return false,
                    Literal::Boolean(b) => return *b,
                    _ => return true,
                }
            }
        }

        return true;
    }


    pub fn writer(&self, value: &Box<dyn Any>) {
        if let Some(val) = value.downcast_ref::<Literal>() {
            match val {
                Literal::Nil => println!("nil"),
                _ => (),
            }
        }
        else if let Some(n) = value.downcast_ref::<f64>() {
            println!("{}", n);
        }
        else if let Some(n) = value.downcast_ref::<String>() {
            println!("{}", n);
        }
        else if let Some(n) = value.downcast_ref::<bool>() {
            println!("{}", n);
        }
        else {
            println!("not implemented");
        }
    }
}