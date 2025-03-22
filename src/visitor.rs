use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt};

pub trait ExprVisitor {
    fn visit_literal(&self, lit: &Literal) -> Result<Box<Literal>, String>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Result<Box<Literal>, String>;
    fn visit_unary(&mut self, op: &UnaryOp, un: &Box<Expr>) -> Result<Box<Literal>, String>;
    fn visit_binary(&mut self, op: &BinaryOp, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<Literal>, String>;
    fn visit_variable(&mut self, s: &String) -> Result<Box<Literal>, String>;
    fn visit_assign(&mut self, t: &String, v: &Box<Expr>) -> Result<Box<Literal>, String>;

}

pub trait ExprAccept {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<Literal>, String>;
}

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), String>;
    fn visit_print_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), String>;
    fn visit_declaration(&mut self, id: &String, initializer: &Option<Box<Expr>>) -> Result<(), String>;
    fn visit_block(&mut self, v: &Box<Vec<Stmt>>) -> Result<(), String>;
    fn visit_if(&mut self, expr: &Box<Expr>, fi: &Box<Stmt>, esl: &Option<Box<Stmt>>) -> Result<(), String>;
}

pub trait StmtAccept {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<(), String>;
}