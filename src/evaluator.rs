use std::{any::Any, process};
use crate::environment::Environment;

use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt, visitor::{ExprAccept, ExprVisitor, StmtAccept, StmtVisitor}};

pub struct Evaluator {
    env: Environment,
}

impl ExprVisitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Result<Box<dyn Any>, String> {
        match expr {
            Literal::Nil => Ok(Box::new(Literal::Nil)),
            Literal::Boolean(b) => Ok(Box::from(*b)),
            Literal::Number(n) => Ok(Box::from(*n)),
            Literal::Str(str) => Ok(Box::new(str.clone())),
        }
    }

    fn visit_grouping(&mut self, box_expr: &Box<Expr>) -> Result<Box<dyn Any>, String> {
        self.evaluate(&box_expr)
    }
    
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Result<Box<dyn Any>, String> {
        let _r = self.evaluate(&un)?;
        match op {
            UnaryOp::Negate => {
                let _num = _r.downcast_ref::<f64>();
                if let Some(num) = _num {
                    return Ok(Box::new(-num));
                }
                else {
                    return Err("Operand must be a number.".to_string());
                }
            },
            UnaryOp::Not => {
                return Ok(Box::new(!self.is_truthy(&_r)));
            }
        }
    }

    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<dyn Any>, String> {
        let _op_left = self.evaluate(&left)?;
        let _op_right = self.evaluate(&right)?;

        match op {
            BinaryOp::Add => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l + r))
                }
                let (vl, vr) = (_op_left.downcast_ref::<String>(), _op_right.downcast_ref::<String>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    let mut res = String::from(l);
                    res.push_str(r);
                    return Ok(Box::new(res));
                }
                Err("Operands must be two numbers or two strings.".to_string())
            }
            BinaryOp::Subtract => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l - r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Multiply => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l * r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Divide => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    if *r == 0.0 {
                        process::exit(65);
                    }
                    return Ok(Box::new(l / r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Greater => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l > r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::GreaterEqual => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l >= r))
                }
                Err("Operands must be numbers.".to_string())

            }
            BinaryOp::Less => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l < r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::LessEqual => {
                let (vl, vr) = (_op_left.downcast_ref::<f64>(), _op_right.downcast_ref::<f64>());
                if let (Some(l), Some(r)) = (vl, vr) {
                    return Ok(Box::new(l <= r))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::EqualEqual => {
                return Ok(Box::new(self.is_equal(&_op_left, &_op_right)));
            }
            BinaryOp::NotEquals => {
                return Ok(Box::new(!self.is_equal(&_op_left, &_op_right)));
            }
            _ => return Err(String::new()),
        }

    }
    
    fn visit_variable(&mut self, name: &String) -> Result<Box<dyn Any>, String> {
        let a= self.env.get(name)?;

        if let Some(_val) = a {
            if let Some(b) = duplicate_boxed_any(_val) {
                return Ok(b);
            }
        }

        return Err("No such variable".to_string());
    }

}

impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Box<dyn Any>, String> {
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
            return false;
        }

        if let Some(b) = r.downcast_ref::<bool>() {
            return *b;
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

impl StmtVisitor for Evaluator  {
    fn visit_expression_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), String> {
        match self.evaluate(&stmt) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }
    
    fn visit_print_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), String> {
        match self.evaluate(&stmt) {
            Ok(d) => Ok(self.writer(&d)),
            Err(e) => Err(e),
        }
    }
    
    fn visit_declaration(&mut self, id: &String, initializer: &Option<Expr>) -> Result<(), String> {
        
        let value = if let Some(expr) = initializer {
            Some(self.evaluate(expr)?)
        }
        else {
            Some(Box::from(Literal::Nil) as Box<dyn Any>)
        };

        self.env.define(id.to_string(), value);
        Ok(())
    }
    
}

impl Evaluator {
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> { 
        for stmt in stmts {
            match self.execute(&stmt) {
                Ok(_) => (),
                Err(e) => {
                    eprintln!("{}", e);
                    process::exit(70);
                }
            }
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        stmt.accept(self)
    }

    pub fn new(env: Environment) -> Self {
        Evaluator {
            env
        }
    }
}

fn duplicate_boxed_any(input: &Box<dyn Any>) -> Option<Box<dyn Any>> {
    if let Some(value) = input.downcast_ref::<f64>() {
        Some(Box::new(*value))
    } 
    else if let Some(value) = input.downcast_ref::<String>() {
        Some(Box::new(value.clone()))
    }
    else if let Some(value) = input.downcast_ref::<bool>() {
        Some(Box::new(*value))
    }
    else if let Some(_) = input.downcast_ref::<Literal>() {
        Some(Box::new(Literal::Nil))
    }
    else {
        None
    }
}