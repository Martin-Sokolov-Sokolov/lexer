use crate::{evaluator::RuntimeException, expr::Expr, token::Token, visitor::{StmtAccept, StmtVisitor}};

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt{
    ExprStmt(Box<Expr>),
    PrintStmt(Box<Expr>),
    Declaration{id: String, initializer: Option<Box<Expr>>},
    Block(Box<Vec<Stmt>>),
    Function(Box<FunctionStmt>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    While(Box<Expr>, Box<Stmt>),
    Return(Box<Token>, Option<Box<Expr>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionStmt {
    pub name: Token,
    pub params: Vec<Token>,
    pub body: Vec<Stmt>,
}

impl FunctionStmt {
    pub fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        FunctionStmt { name, params, body }
    }
}

impl StmtAccept for Stmt  {
    fn accept <'a> (&self, visitor: &'a mut dyn StmtVisitor) -> Result<(), RuntimeException> {
        match self {
            Stmt::ExprStmt(es) => visitor.visit_expression_stmt(es),
            Stmt::PrintStmt(ps) => visitor.visit_print_stmt(ps),
            Stmt::Declaration { id, initializer } => visitor.visit_declaration(id, initializer),
            Stmt::Block(v) => visitor.visit_block(v),
            Stmt::If(cond, fi, esl) => visitor.visit_if(cond, fi, esl),
            Stmt::While(expr, st) => visitor.visit_while(expr, st),
            Stmt::Function(fun_stmt) => visitor.visit_function(fun_stmt),
            Stmt::Return(tok, exp) => visitor.visit_return(tok, exp),
        }
    }
}