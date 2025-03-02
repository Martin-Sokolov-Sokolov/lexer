use crate::expr::Expr;

pub enum Stmt {
    ExprStmt(Expr),
    PrintStmt(Expr),
}

