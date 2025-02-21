use std::ptr::null;

use crate::scanner::*;

enum Expr <'a> {
    Lit(Literal<'a>),
    Unary(UnaryOp, Box<Expr<'a>>),
    Binary(Box<Expr<'a>>, BinaryOp, Box<Expr<'a>>),
    BinaryOp,
    Grouping(Box<Expr<'a>>),
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


enum Literal <'a> {
    Number(f64),
    Str(&'a str),
    True(bool),
    False(bool),
    Nil,
}

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

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let expr = self.comparison();

        expr
    }

    fn comparison(&mut self) -> Expr {
        let expr = self.term();

        expr
    }

    fn term(&mut self) -> Expr {
        let expr = self.factor();

        expr
    }

    fn factor(&mut self) -> Expr {
        let expr = self.unary();

        expr
    }

    fn unary(&self) -> Expr {
        Expr::BinaryOp
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_at_end() && &self.peek().token_type != token_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token_type: &TokenType, err: String) -> Result<&Token, String> {
        if self.check(token_type) {
            Ok(self.advance())
        }
        else {
            Err(err)
        }
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current-1).unwrap()
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len()
    }

    fn primary<'a>(&mut self) -> Expr {
        if self.mat(Vec::from([TokenType::False])) {return Expr::Lit(Literal::False(false)); }
        else if self.mat(Vec::from([TokenType::True])) {return Expr::Lit(Literal::True(true)); }
        else if self.mat(Vec::from([TokenType::Nil])) { return Expr::Lit(Literal::Nil); }

        else if self.mat(Vec::from([TokenType::String])) {
            if let Some(lit) = &self.previous().literal {
                if let Some(str_val) = lit.downcast_ref::<String>() {
                    return Expr::Lit(Literal::Str(&str_val.as_str()));
                }
            }

        }
        else if self.mat(Vec::from([TokenType::LeftParen])) {
            let expr = self.expression();
            return Expr::Grouping(Box::from(expr));
        }
        else {
            let t = self.peek();
            if let TokenType::Number(n) = t.token_type {
                if self.mat(Vec::from([TokenType::Number(n)])) {
                    if let Some(lit) = &self.previous().literal {
                        if let Some(num_val) = lit.downcast_ref::<f64>() {
                            return Expr::Lit(Literal::Number(*num_val));
                        }
                    }
                }
            }
        }
        
        Expr::Lit(Literal::Nil)
    }

    fn mat(&mut self, v: Vec<TokenType>) -> bool {
        for token_type in v {
            if self.check(&token_type) {
                self.advance();
            }
            return true;
        }
        false
    }

}


