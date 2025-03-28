use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{any::Any, process};

use crate::environment::Environment;

use crate::lox_callable::LoxCallable;
use crate::token::{Token, TokenType};
use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt, visitor::{ExprAccept, ExprVisitor, StmtAccept, StmtVisitor}};

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
    globals: Rc<RefCell<Environment>>,
}


impl LoxCallable for Literal {
    fn callq(&self, _: &mut Evaluator, _: Vec<Expr>) -> Box<Literal> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        Box::from(Literal::Number(now.as_secs_f64()))
    }
    
    fn arrity(&self) -> usize {
        0
    }

    fn to_string(&self) -> String {
        "<native fn>".to_string()
    }

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
        let a= self.env.borrow().get(var_name)?;

        if let Some(_val) = a {
            return Ok(Box::from(*_val.clone()));
        }

        return Err("No such variable".to_string());
    }
    
    fn visit_assign(&mut self, s: &String, a: &Box<Expr>) -> Result<Box<Literal>, String> {
        let val = self.evaluate(&**a)?;
        self.env.borrow_mut().assign(s, Some(&val))?;
        return Ok(val);
    }
    
    fn visit_logical(&mut self, left: &Box<Expr>, op: &Box<Token>, right: &Box<Expr>) -> Result<Box<Literal>, String> {
        let l = self.evaluate(left)?;

        if let TokenType::Or = &op.token_type {
            if self.is_truthy(&l) {
                return Ok(l);
            } 
        }
        else if let TokenType::And = &op.token_type {
            if !self.is_truthy(&l) {
                return Ok(l);
            }
        }

        self.evaluate(&right)
    }
    
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Box<Token>, arguments: &Box<Vec<Expr>>) -> Result<Box<Literal>, String> {
        let cal = self.evaluate(&callee)?;

        let mut args= Vec::new();

        for arg in arguments.iter() {
            args.push(*self.evaluate(arg)?);
        }

        if args.len() != cal.arrity() {
            let err = format!("Expected {} arguments but got {}.", cal.arrity(), args.len());
            return Err(err);
        }


        Ok(cal.callq(self, arguments.clone().to_vec()))
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
    
    fn visit_declaration(&mut self, id: &String, initializer: &Option<Box<Expr>>) -> Result<(), String> {
        let value = if let Some(expr) = initializer {
            Some(self.evaluate(expr)?)
        }
        else {
            Some(Box::from(Literal::Nil) as Box<Literal>)
        };

        self.env.borrow_mut().define(id.to_string(), value);
        Ok(())
    }
    
    fn visit_block(&mut self, v: &Box<Vec<Stmt>>) -> Result<(), String> {
        let new_env = Rc::new(RefCell::new(Environment::new_enclosing(self.env.clone())));
        self.execute_block(&v, new_env)
    }
    
    fn visit_if(&mut self, expr: &Box<Expr>, fi: &Box<Stmt>, esl: &Option<Box<Stmt>>) -> Result<(), String> {
        let c = self.evaluate(expr)?;

        if self.is_truthy(&c) {
            self.execute(&**fi)?;
        }
        else if let Some(else_val) = esl {
            self.execute(&**else_val)?;
        }

        Ok(())
    }
    
    fn visit_while(&mut self, expr: &Box<Expr>, st: &Box<Stmt>) -> Result<(), String> {
        let mut cond = self.evaluate(expr)?;

        while self.is_truthy(&cond) {
            self.execute(&st)?;
            cond = self.evaluate(expr)?;
        }

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

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, new_env: Rc<RefCell<Environment>>) -> Result<(), String> {
        let previous = self.env.clone();
        self.env = new_env;
        
        let result = statements.iter().try_for_each(|st| self.execute(st));
        
        self.env = previous;
        result
    }
    

    fn execute(&mut self, stmt: &Stmt) -> Result<(), String> {
        stmt.accept(self)
    }

    pub fn new(globals: Rc<RefCell<Environment>>) -> Self {
        let b: Option<Box<Literal>>= None;
        globals.borrow_mut().define("clock".to_string(), Some(Box::from(Literal::Nil)));
        Evaluator {
            env: globals.clone(),
            globals
        }
    }
}