use std::any::Any;

use crate::expr::{BinaryOp, Expr, Literal, UnaryOp};

pub trait Visitor {
    fn visit_literal(&self, lit: &Literal) -> Result<Box<dyn Any>, String>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Result<Box<dyn Any>, String>;
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Result<Box<dyn Any>, String>;
    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<dyn Any>, String>;
}

pub trait Accept {
    fn accept(&self, visitor: &mut dyn Visitor) -> Result<Box<dyn Any>, String>;
}