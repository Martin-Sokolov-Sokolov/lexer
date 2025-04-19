use std::{borrow::Cow, fmt};

use crate::{evaluator::RuntimeException, lox_function::{LoxAnonymous, LoxFunction}, token::Token, visitor::{ExprAccept, ExprVisitor}};

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
    Unary(Box<Token>, Box<Expr>),
    Binary(Box<Expr>, Box<Token>, Box<Expr>),
    Call(Box<Expr>, Box<Token>, Box<Vec<Expr>>),
    Grouping(Box<Expr>),
    Variable(Box<Token>),
    Assign(Box<Token>, Box<Expr>),
    Logical(Box<Expr>, Box<Token>, Box<Expr>),
}

impl ExprAccept for Expr {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<Literal>, RuntimeException> {
        match self {
            Expr::Lit(l) => visitor.visit_literal(l),
            Expr::Grouping(gr) => visitor.visit_grouping(gr),
            Expr::Unary(op, b) => visitor.visit_unary(op, b),
            Expr::Binary(left, op, right) => visitor.visit_binary(op, left, right),
            Expr::Variable(name) => visitor.visit_variable(name),
            Expr::Assign(name, v) => visitor.visit_assign(name, v),
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
            Expr::Binary(left, operator, right) => write!(f, "({} {} {})", operator.lexeme, left, right),
            Expr::Unary(operator, right) => write!(f, "({} {})", operator.lexeme, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
            Expr::Variable(s) => write!(f, "{}", s),
            Expr::Assign(t, _) => write!(f, "{}", t),
            Expr::Logical(_, _, _) => write!(f, ""),
            Expr::Call(_, _, _) => write!(f, "ads"), 
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOp {
    Negate,
    Not,   
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
