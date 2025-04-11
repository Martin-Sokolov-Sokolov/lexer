use std::{borrow::Cow, fmt, path::Display};

use crate::{lox_function::{LoxAnonymous, LoxFunction}, token::{Token, TokenType}, visitor::{ExprAccept, ExprVisitor}};

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(f64),
    Str(String),
    Boolean(bool),
    LoxCallable(LoxCallables),
    Nil,
}

#[derive(Debug, Clone, PartialEq)]
pub enum LoxCallables {
    LoxFunction(Box<LoxFunction>),
    LoxAnonymous(Box<LoxAnonymous>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Lit(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(Box<Expr>, Box<Token>, Box<Vec<Expr>>),
    Grouping(Box<Expr>),
    Variable(String),
    Assign(String, Box<Expr>),
    Logical(Box<Expr>, Box<Token>, Box<Expr>),
}

impl ExprAccept for Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<Literal>, String> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            Expr::Grouping(gr) => visitor.visit_grouping(gr),
            Expr::Unary(op, b) => visitor.visit_unary(op, b),
            Expr::Binary(left, op, right) => visitor.visit_binary(op, left, right),
            Expr::Variable(s) => visitor.visit_variable(s),
            Expr::Assign(t, v) => visitor.visit_assign(t, v),
            Expr::Logical(left, op, right) => visitor.visit_logical(left, op, right),
            Expr::Call(callee, paren, arguments) => visitor.visit_call(callee, paren, arguments),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Lit(Literal::Boolean(b)) => write!(f, "{}", b),
            Expr::Lit(Literal::Nil) => write!(f, "nil"),
            Expr::Lit(Literal::Str(s)) => write!(f, "{}", unescape(s)),
            Expr::Lit(Literal::Number(n)) => write!(f, "{n:?}"),
            Expr::Lit(Literal::LoxCallable(lc)) => write!(f, "{lc}"), 
            Expr::Binary(left, operator, right) => write!(f, "({} {} {})", operator, left, right),
            Expr::Unary(operator, right) => write!(f, "({} {})", operator, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Variable(s) => write!(f, "{}", s),
            Expr::Assign(t, _) => write!(f, "{}", t),
            Expr::Logical(_, _, _) => write!(f, ""),
            Expr::Call(a, b, c) => write!(f, "ads"), 
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Negate,
    Not,   
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl UnaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<UnaryOp> {
        match token_type {
            TokenType::Minus => Some(UnaryOp::Negate),
            TokenType::Bang => Some(UnaryOp::Not),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
    Equals,
    EqualEqual,
    NotEquals,   
    Less,        
    LessEqual,   
    Greater,     
    GreaterEqual,
    Add,         
    Subtract,    
    Multiply,    
    Divide,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOp::Equals => "=",
            BinaryOp::EqualEqual => "==",
            BinaryOp::NotEquals => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        };
        write!(f, "{}", symbol)
    }
}

impl BinaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<BinaryOp> {
        match token_type {
            TokenType::Equal => Some(BinaryOp::Equals),
            TokenType::EqualEqual => Some(BinaryOp::EqualEqual),
            TokenType::BangEqual => Some(BinaryOp::NotEquals),
            TokenType::Less => Some(BinaryOp::Less),
            TokenType::LessEqual => Some(BinaryOp::LessEqual),
            TokenType::Greater => Some(BinaryOp::Greater),
            TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Subtract),
            TokenType::Star => Some(BinaryOp::Multiply),
            TokenType::Slash => Some(BinaryOp::Divide),
            _ => None,
        }
    }
}

impl fmt::Display for LoxCallables {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoxCallables::LoxFunction(lc) => write!(f, "<fn {}>", lc.declaration.name.lexeme),
            LoxCallables::LoxAnonymous(la) => write!(f, "<anonymous fn>"),
        }
    }
}

pub fn unescape(s: &str) -> Cow<str> {
    Cow::Borrowed(s.trim_matches('"'))
}
