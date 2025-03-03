use std::any::Any;

use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt};

pub trait ExprVisitor {
    fn visit_literal(&self, lit: &Literal) -> Result<Box<dyn Any>, String>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Result<Box<dyn Any>, String>;
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Result<Box<dyn Any>, String>;
    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<dyn Any>, String>;
}

pub trait ExprAccept {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<dyn Any>, String>;
}

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Box<Expr>);
    fn visit_print_stmt(&mut self, stmt: &Box<Expr>);
}

pub trait StmtAccept {
    fn accept(&self, visitor: &mut dyn StmtVisitor);
}