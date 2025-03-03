use std::any::Any;

use crate::{expr::Expr, visitor::{StmtAccept, StmtVisitor}};

pub enum Stmt {
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
}

impl StmtAccept for Stmt {
    fn accept(&self, visitor: &mut dyn StmtVisitor) {
        match self {
            Stmt::ExprStmt(es) => visitor.visit_expression_stmt(es),
            Stmt::PrintStmt(ps) => visitor.visit_print_stmt(ps),
        }
    }
}