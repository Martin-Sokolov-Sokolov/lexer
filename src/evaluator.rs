use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{any::Any, process};

use crate::environment::Environment;

use crate::expr::LoxCallables;
use crate::lox_callable::LoxCallable;
use crate::lox_function::{LoxAnonymous, LoxFunction};
use crate::stmt::FunctionStmt;
use crate::token::{Token, TokenType};
use crate::{expr::{Expr, Literal}, stmt::Stmt, visitor::{ExprAccept, ExprVisitor, StmtAccept, StmtVisitor}};

pub struct Evaluator {
    env: Rc<RefCell<Environment>>,
    pub globals: Rc<RefCell<Environment>>,
}


impl ExprVisitor for Evaluator {
    fn visit_literal(&self, expr: &Literal) -> Result<Box<Literal>, RuntimeException> {
        match expr {
            Literal::Nil => Ok(Box::new(Literal::Nil)),
            Literal::Boolean(b) => Ok(Box::from(Literal::Boolean(*b))),
            Literal::Number(n) => Ok(Box::from(Literal::Number(*n))),
            Literal::Str(str) => Ok(Box::new(Literal::Str(String::from(str)))),
            Literal::LoxCallable(lc) => Ok(Box::from(Literal::LoxCallable(lc.clone()))),
        }
    }

    fn visit_grouping(&mut self, box_expr: &Box<Expr>) -> Result<Box<Literal>, RuntimeException> {
        self.evaluate(&box_expr)
    }
    
    fn visit_unary(&mut self, op: &Box<Token>, un: &Box<Expr>) -> Result<Box<Literal>, RuntimeException> {
        let _r = self.evaluate(&un)?;
        match (*op).token_type {
            TokenType::Minus => {
                if let Literal::Number(num) = *_r {
                    return Ok(Box::new(Literal::Number(-num)));
                }
                else {
                    return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operand must be a number.")));
                }
            },
            TokenType::Bang => {
                return Ok(Box::new(Literal::Boolean(!self.is_truthy(&_r))));
            }
            _ => return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operand must be a number."))),
        }
    }

    fn visit_binary(&mut self, op: &Box<Token>, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<Literal>, RuntimeException> {
        let _op_left = self.evaluate(&left)?;
        let _op_right = self.evaluate(&right)?;

        match (*op).token_type {
            TokenType::Plus => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l + *r)))
                }
                if let (Literal::Str(l), Literal::Str(r)) = (&*_op_left, &*_op_right) {
                    let mut res = String::from(l);
                    res.push_str(&r);
                    return Ok(Box::new(Literal::Str(res)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operands must be two numbers or two strings.")));
            }
            TokenType::Minus => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l - *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operand must be a number.")));
            }
            TokenType::Star => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Number(*l * *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operand must be a number.")));
            }
            TokenType::Slash => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    if *r == 0.0 {
                        process::exit(65);
                    }
                    return Ok(Box::new(Literal::Number(*l / *r)))
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operand must be a number.")));
            }
            TokenType::Greater => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l > *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operands must be numbers.")));
            }
            TokenType::GreaterEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l >= *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operands must be numbers.")));
            }
            TokenType::Less => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l < *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operands must be numbers.")));
            }
            TokenType::LessEqual => {
                if let (Literal::Number(l), Literal::Number(r)) = (&*_op_left, &*_op_right) {
                    return Ok(Box::new(Literal::Boolean(*l <= *r)));
                }
                return Err(RuntimeException::RuntimeError(RuntimeError::new(op, "Operands must be numbers.")));
            }
            TokenType::EqualEqual => {
                return Ok(Box::new(Literal::Boolean(self.is_equal(&_op_left, &_op_right))));
            }
            TokenType::BangEqual => {
                return Ok(Box::new(Literal::Boolean(!self.is_equal(&_op_left, &_op_right))));
            }
            _ => return Err(RuntimeException::RuntimeError(RuntimeError::new(op, ""))),
        }

    }
    
    fn visit_variable(&mut self, name: &Box<Token>) -> Result<Box<Literal>, RuntimeException> {
        let a= self.env.borrow().get(&name)?;

        if let Some(_val) = a {
            return Ok(Box::from(*_val.clone()));
        }

        return Err(RuntimeException::RuntimeError(RuntimeError::new(&name, "")));
    }
    
    fn visit_assign(&mut self, name: &Box<Token>, a: &Box<Expr>) -> Result<Box<Literal>, RuntimeException> {
        let val = self.evaluate(&**a)?;
        self.env.borrow_mut().assign(*&name, Some(&val))?;
        return Ok(val);
    }
    
    fn visit_logical(&mut self, left: &Box<Expr>, op: &Box<Token>, right: &Box<Expr>) -> Result<Box<Literal>, RuntimeException> {
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
    
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Box<Token>, arguments: &Box<Vec<Expr>>) -> Result<Box<Literal>, RuntimeException> {
        let callee = *self.evaluate(&callee)?;

        let mut args= Vec::new();
        for arg in arguments.iter() {
            args.push(*self.evaluate(arg)?);
        }

        let function = match callee {
            Literal::LoxCallable(lit) => Ok(lit),
            _ => Err(RuntimeException::RuntimeError(RuntimeError::new(&paren, ""))),
        };

        if args.len() != function.clone()?.arrity() {
            return Err(RuntimeException::RuntimeError(RuntimeError::new(&paren, &format!("Expected {} arguments but got {}.", function?.clone().arrity(), arguments.len()))));
        }

        let res = function?.callq(self, args);

        return match res {
            Err(RuntimeException::Return(value)) => Ok(Box::from(value.value.unwrap())),
            Ok(Some(val)) => Ok(val),
            Ok(None) => Ok(Box::from(Literal::Nil)),
            Err(e) => Err(e),
        };
    }
    
}


impl Evaluator {
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Box<Literal>, RuntimeException> {
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
            Literal::LoxCallable(lc) => println!("{}", lc),
        }
    }
}

impl StmtVisitor for Evaluator  {
    fn visit_expression_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), RuntimeException> {
        self.evaluate(&stmt)?;
        Ok(())
    }
    
    fn visit_print_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), RuntimeException> {
        match self.evaluate(&stmt) {
            Ok(d) => Ok(self.writer(&d)),
            Err(e) => Err(e),
        }
    }
    
    fn visit_declaration(&mut self, id: &String, initializer: &Option<Box<Expr>>) -> Result<(), RuntimeException> {
        let value = if let Some(expr) = initializer {
            Some(self.evaluate(expr)?)
        }
        else {
            Some(Box::from(Literal::Nil) as Box<Literal>)
        };

        self.env.borrow_mut().define(id.to_string(), value);
        Ok(())
    }
    
    fn visit_block(&mut self, v: &Box<Vec<Stmt>>) -> Result<(), RuntimeException> {
        let new_env = Rc::new(RefCell::new(Environment::new(Some(self.env.clone()))));
        self.execute_block(&v, new_env)
    }
    
    fn visit_if(&mut self, expr: &Box<Expr>, fi: &Box<Stmt>, esl: &Option<Box<Stmt>>) -> Result<(), RuntimeException> {
        let c = self.evaluate(expr)?;

        if self.is_truthy(&c) {
            self.execute(&**fi)?;
        }
        else if let Some(else_val) = esl {
            self.execute(&**else_val)?;
        }

        Ok(())
    }
    
    fn visit_while(&mut self, expr: &Box<Expr>, st: &Box<Stmt>) -> Result<(), RuntimeException> {
        let mut cond = self.evaluate(expr)?;

        while self.is_truthy(&cond) {
            self.execute(&st)?;
            cond = self.evaluate(expr)?;
        }

        Ok(())
    }
    
    fn visit_function(&mut self, fun_stmt: &Box<FunctionStmt>) -> Result<(), RuntimeException> {
        let function = LoxFunction::new(*fun_stmt.clone(), self.env.clone());
        self.env.borrow_mut().define(fun_stmt.name.lexeme.clone(),
                                    Some(Box::from(Literal::LoxCallable(LoxCallables::LoxFunction(Box::from(function))))));
        Ok(())
    }
    
    fn visit_return(&mut self, _tok: &Box<Token>, exp: &Option<Box<Expr>>) -> Result<(), RuntimeException> {
        let value = match exp {
            Some(expr) => self.evaluate(expr)?,
            None => Box::from(Literal::Nil),
        };
        
        return Err(RuntimeException::Return(Return::new(Some(*value))));
    }
    
    
}

impl Evaluator {
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), RuntimeException> { 
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

    pub fn execute_block(&mut self, statements: &Vec<Stmt>, new_env: Rc<RefCell<Environment>>) -> Result<(), RuntimeException> {
        let previous = self.env.clone();
        self.env = new_env;
        
        let result = statements.iter().try_for_each(|st| self.execute(st));
        
        self.env = previous;
        result
    }
    

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeException> {
        stmt.accept(self)
    }

    pub fn new(globals: Rc<RefCell<Environment>>) -> Self {
        globals.borrow_mut().define("clock".to_owned(),
            Some(Box::from(Literal::LoxCallable(LoxCallables::LoxAnonymous(
                Box::new(LoxAnonymous::new(|_interpreter, _arguments| {
                    Ok(Some(Box::from(Literal::Number(
                        SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs_f64(),
                    ))))
                }, || 0,
                )),
            ))),
        ));
        Evaluator {
            env: globals.clone(),
            globals
        }
    }
}

#[derive(Clone)]
pub struct RuntimeError {
    pub token: Token,
    pub message: String,
}

impl RuntimeError {
    pub fn new(token: &Token, message: &str) -> Self {
        return RuntimeError {
            token: token.clone(),
            message: message.to_string(),
        };
    }
}

#[derive(Clone)]
pub struct Return {
    pub value: Option<Literal>,
}

impl Return {
    pub fn new(value: Option<Literal>) -> Self {
        Self { value }
    }
}

#[derive(Clone)]
pub enum RuntimeException {
    RuntimeError(RuntimeError),
    Return(Return),
}

impl fmt::Display for RuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeException::RuntimeError(e) => write!(f, "{}", e.message),
            _ => write!(f, ""),
        }
    }
}