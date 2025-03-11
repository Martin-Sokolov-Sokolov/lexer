use std::{any::Any, borrow::Cow, fmt};

use crate::{token::TokenType, visitor::{ExprAccept, ExprVisitor}};



impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Lit(Literal::Boolean(b)) => write!(f, "{}", b),
            Expr::Lit(Literal::Nil) => write!(f, "nil"),
            Expr::Lit(Literal::Str(s)) => write!(f, "{}", unescape(s)),
            Expr::Lit(Literal::Number(n)) => write!(f, "{n:?}"),
            Expr::Binary(left, operator, right) => write!(f, "({} {} {})", operator, left, right),
            Expr::Unary(operator, right) => write!(f, "({} {})", operator, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Variable(s) => write!(f, "{}", s),
            Expr::Assign(t, _) => write!(f, "{}", t),
        }
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum Literal {
    Number(f64),
    Str(String),
    Boolean(bool),
    Nil,
}

pub fn unescape(s: &str) -> Cow<str> {
    Cow::Borrowed(s.trim_matches('"'))
}

#[derive(Debug)]
pub enum Expr {
    Lit(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(String),
    Assign(String, Box<Expr>),
}

impl ExprAccept for Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<dyn Any>, String> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            Expr::Grouping(gr) => visitor.visit_grouping(gr),
            Expr::Unary(op, b) => visitor.visit_unary(op, b),
            Expr::Binary(left, op, right) => visitor.visit_binary(op, left, right),
            Expr::Variable(s) => visitor.visit_variable(s),
            Expr::Assign(t, v) => visitor.visit_assign(t, v),
        }
    }
}