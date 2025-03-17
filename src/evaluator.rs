use std::{any::Any, process};

use crate::environment::Environment;

use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt, visitor::{ExprAccept, ExprVisitor, StmtAccept, StmtVisitor}};

pub struct Evaluator {
    env: Environment,
}

impl ExprVisitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Result<Box<Literal>, String> {
        match expr {
            Literal::Nil => Ok(Box::new(Literal::Nil)),
            Literal::Boolean(b) => Ok(Box::from(Literal::Boolean(*b))),
            Literal::Number(n) => Ok(Box::from(Literal::Number(*n))),
            Literal::Str(str) => Ok(Box::new(Literal::Str(String::from(str)))),
        }
    }

    fn visit_grouping(&mut self, box_expr: &Box<Expr>) -> Result<Box<Literal>, String> {
        self.evaluate(&box_expr)
    }
    
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Result<Box<Literal>, String> {
        let _r = self.evaluate(&un)?;
        match op {
            UnaryOp::Negate => {
                if let Literal::Number(num) = *_r {
                    return Ok(Box::new(Literal::Number(-num)));
                }
                else {
                    return Err("Operand must be a number.".to_string());
                }
            },
            UnaryOp::Not => {
                return Ok(Box::new(Literal::Boolean(!self.is_truthy(&_r))));
            }
        }
    }

    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<Literal>, String> {
        let _op_left = self.evaluate(&left)?;
        let _op_right = self.evaluate(&right)?;

        match op {
            BinaryOp::Add => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l + *r)))
                }
                if let (Literal::Str(l), Literal::Str(r)) = (&*_op_left, &*_op_right) {
                    let mut res = String::from(l);
                    res.push_str(&r);
                    return Ok(Box::new(Literal::Str(res)));
                }
                Err("Operands must be two numbers or two strings.".to_string())
            }
            BinaryOp::Subtract => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l - *r)));
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Multiply => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l * *r)));
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Divide => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    if *r == 0.0 {
                        process::exit(65);
                    }
                    return Ok(Box::new(Literal::Number(*l / *r)))
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::Greater => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l > *r)));
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::GreaterEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l >= *r)));
                }
                Err("Operands must be numbers.".to_string())

            }
            BinaryOp::Less => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l < *r)));
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::LessEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l <= *r)));
                }
                Err("Operands must be numbers.".to_string())
            }
            BinaryOp::EqualEqual => {
                return Ok(Box::new(Literal::Boolean(self.is_equal(&_op_left, &_op_right))));
            }
            BinaryOp::NotEquals => {
                return Ok(Box::new(Literal::Boolean(!self.is_equal(&_op_left, &_op_right))));
            }
            _ => return Err(String::new()),
        }

    }
    
    fn visit_variable(&mut self, var_name: &String) -> Result<Box<Literal>, String> {
        let a= self.env.get(var_name)?;

        if let Some(_val) = a {
            return Ok(Box::from(*_val.clone()));
        }

        return Err("No such variable".to_string());
    }
    
    fn visit_assign(&mut self, s: &String, a: &Box<Expr>) -> Result<Box<Literal>, String> {
        let val = self.evaluate(&**a)?;
        self.env.assign(s, Some(val.clone()))?;
        return Ok(val);
    }
    
}


impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Box<Literal>, String> {
        expr.accept(self)
    }

    fn is_equal(&self, l: &Box<Literal>, r: &Box<Literal>) -> bool {
        if l.type_id() != r.type_id() {
            return false;
        }
        match (&**l, &**r) {
            (Literal::Boolean(l_val), Literal::Boolean(r_val)) => return *l_val == *r_val,
            (Literal::Number(l_val), Literal::Number(r_val)) => return *l_val == *r_val,
            (Literal::Str(l_val), Literal::Str(r_val)) => return *l_val == *r_val,
            _ => return false,
        }
    }

    pub fn is_truthy(&self, val: &Box<Literal>) -> bool {
        if **val == Literal::Nil {
            return false;
        }

        if let Literal::Boolean(bool_val) = &**val {
            return *bool_val;
        }

        return true;
    }


    pub fn writer(&self, value: &Box<Literal>) {
        match &**value {
            Literal::Nil => println!("nil"),
            Literal::Boolean(val) => println!("{}", val),
            Literal::Number(val) => println!("{}", val),
            Literal::Str(val) => println!("{}", val),
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
            Some(Box::from(Literal::Nil) as Box<Literal>)
        };

        self.env.define(id.to_string(), value);
        Ok(())
    }
    
    fn visit_block(&mut self, v: &Box<Vec<Stmt>>) -> Result<(), String> {
        self.execute_block(&v, Environment::new_enclosing(&self.env))
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

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, env: Environment) -> Result<(), String>{
        let previous = env.clone();
        self.env = env;
        for st in statements {
            self.execute(st)?;
        }
        self.env = previous;
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