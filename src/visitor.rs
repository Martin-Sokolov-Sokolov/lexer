use crate::{evaluator::RuntimeException, expr::{Expr, Literal}, stmt::{FunctionStmt, Stmt}, token::Token};

pub trait ExprVisitor {
    fn visit_literal(&self, lit: &Literal) -> Result<Box<Literal>, RuntimeException>;
    fn visit_grouping(&mut self, gr: &Box<Expr>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_unary(&mut self, op: &Box<Token>, un: &Box<Expr>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_binary(&mut self, op: &Box<Token>, left: &Box<Expr>, right: &Box<Expr>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_variable(&mut self, name: &Box<Token>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_assign(&mut self, name: &Box<Token>, v: &Box<Expr>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_logical(&mut self, left: &Box<Expr>, op: &Box<Token>, right: &Box<Expr>) -> Result<Box<Literal>, RuntimeException>;
    fn visit_call(&mut self, callee: &Box<Expr>, paren: &Box<Token>, arguments: &Box<Vec<Expr>>) -> Result<Box<Literal>, RuntimeException>;
}

pub trait ExprAccept {
    fn accept(&self, visitor: &mut dyn ExprVisitor) -> Result<Box<Literal>, RuntimeException>;
}

pub trait StmtVisitor {
    fn visit_expression_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), RuntimeException>;
    fn visit_print_stmt(&mut self, stmt: &Box<Expr>) -> Result<(), RuntimeException>;
    fn visit_declaration(&mut self, id: &String, initializer: &Option<Box<Expr>>) -> Result<(), RuntimeException>;
    fn visit_block(&mut self, v: &Box<Vec<Stmt>>) -> Result<(), RuntimeException>;
    fn visit_if(&mut self, expr: &Box<Expr>, fi: &Box<Stmt>, esl: &Option<Box<Stmt>>) -> Result<(), RuntimeException>;
    fn visit_while(&mut self, expr: &Box<Expr>, st: &Box<Stmt>) -> Result<(), RuntimeException>;
    fn visit_function(&mut self, fun_stmt: &Box<FunctionStmt>) -> Result<(), RuntimeException>;
    fn visit_return(&mut self, tok: &Box<Token>, exp: &Option<Box<Expr>>) -> Result<(), RuntimeException>;
}

pub trait StmtAccept {
    fn accept(&self, visitor: &mut dyn StmtVisitor) -> Result<(), RuntimeException>;
}