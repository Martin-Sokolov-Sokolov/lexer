use crate::scanner::*;

enum Expr {
    Lit(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    BinaryOp,
    Grouping(Box<Expr>),
}

pub enum UnaryOp {
    Negate,
    Not,   
}

pub enum BinaryOp {
    Equals,      
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


enum Literal {
    Number(f64),
    Str(String),
    True(bool),
    False(bool),
    Nil,
}

#[derive(Debug)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(&self, tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
        let expr = self.comparison();


        expr
    }

    fn comparison(&self) -> Expr {
        let expr = self.term();

        expr
    }

    fn term(&self) -> Expr {
        let expr = self.factor();

        expr
    }

    fn factor(&self) -> Expr {
        let expr = self.unary();

        expr
    }

    fn unary(&self) -> Expr {
        Expr::BinaryOp
    }

    fn check(&self, to_compare: Token) -> bool {
        if self.is_at_end() || self.tokens[self.current] != to_compare {
            false
        }
        else {
            true
        }
    }

    fn is_at_end(&self) -> bool{
        self.current >= self.tokens.len()
    }

}


