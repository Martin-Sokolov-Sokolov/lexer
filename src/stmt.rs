use crate::{expr::Expr, visitor::{StmtAccept, StmtVisitor}};

pub enum Stmt{
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
    Declaration{ id: String, initializer: Option<Box<Expr>>},
    Block(Box<Vec<Stmt>>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
}

impl StmtAccept for Stmt  {
    fn accept <'a> (&self, visitor: &'a mut dyn StmtVisitor) -> Result<(), String> {
        match self {
            Stmt::ExprStmt(es) => visitor.visit_expression_stmt(es),
            Stmt::PrintStmt(ps) => visitor.visit_print_stmt(ps),
            Stmt::Declaration { id, initializer } => visitor.visit_declaration(id, initializer),
            Stmt::Block(v) => visitor.visit_block(v),
            Stmt::If(cond, fi, esl) => visitor.visit_if(cond, fi, esl),
        }
    }
}