use crate::scanner::*;
use std::{fmt::{self, Pointer}};
use std::io::{self, Write};

#[derive(Debug)]
pub enum Expr {
    Lit(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    BinaryOp,
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Lit(Literal::False(b)) => write!(f, "{}", b),
            Expr::Lit(Literal::True(b)) => write!(f, "{}", b),
            Expr::Lit(Literal::Nil) => write!(f, "null"),
            _ => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,   
}
impl UnaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<UnaryOp> {
        match token_type {
            TokenType::Minus => Some(UnaryOp::Negate),
            TokenType::Slash => Some(UnaryOp::Not),
            _ => None,
        }
    }
}

#[derive(Debug)]
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

impl BinaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<BinaryOp> {
        match token_type {
            TokenType::EqualEqual => Some(BinaryOp::Equals),
            TokenType::BangEqual => Some(BinaryOp::NotEquals),
            TokenType::Less => Some(BinaryOp::Less),
            TokenType::LessEqual => Some(BinaryOp::LessEqual),
            TokenType::Greater => Some(BinaryOp::Greater),
            TokenType::GreaterEqual => Some(BinaryOp::GreaterEqual),
            TokenType::Plus => Some(BinaryOp::Add),
            TokenType::Minus => Some(BinaryOp::Subtract),
            TokenType::Star => Some(BinaryOp::Multiply),
            TokenType::Slash => Some(BinaryOp::Divide),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Str(String),
    True(bool),
    False(bool),
    Nil,
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            current: 0,
        }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.mat(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.comparison();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn comparison(&mut self) ->  Expr {
        let mut expr = self.term();

        while self.mat(&[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.term();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn term(&mut self) ->  Expr {
        let mut expr = self.factor();

        while self.mat(&[TokenType::Minus, TokenType::Plus]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.factor();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn factor(&mut self) ->  Expr {
        let mut expr = self.unary();

        while self.mat(&[TokenType::Dot, TokenType::Slash]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.unary();
            expr = Expr::Binary(Box::from(expr), operator, Box::from(right));
        }

        expr
    }

    fn unary(&mut self) -> Expr {

        if self.mat(&[TokenType::Minus, TokenType::Bang]) {
            let operator = UnaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.unary();

            return Expr::Unary(operator, Box::from(right));
        }

        self.primary()
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return &self.peek().token_type == token_type;
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
        return self.peek().token_type == TokenType::EOF;
    }

    fn primary (&mut self) -> Expr {
        if self.mat(&[TokenType::False]) {
            let expr = Expr::Lit(Literal::False(false));
            return expr;
        }
        else if self.mat(&[TokenType::True]) {
            let expr =  Expr::Lit(Literal::True(true));
            return expr;
        }
        else if self.mat(&[TokenType::Nil]) { 
            let expr = Expr::Lit(Literal::Nil);
            return expr;
        }

        else if self.mat(&[TokenType::String]) {
            if let Some(lit) = &self.previous().literal {
                if let Some(str_val) = lit.downcast_ref::<String>() {
                    return Expr::Lit(Literal::Str(String::from(str_val)));
                }
            }
        }
        else if self.mat(&[TokenType::LeftParen]) {
            let expr = self.expression();
            let _ = self.consume(&TokenType::RightParen, "Expect ')' after expression.".to_string());
            return Expr::Grouping(Box::from(expr));
        }
        else {
            let t = self.peek();
            if let TokenType::Number(n) = t.token_type {
                if self.mat(&[TokenType::Number(n)]) {
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

    fn mat(&mut self, v: &[TokenType]) -> bool {
        for token_type in v {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut res: Vec<Expr> = vec![];
        while !self.is_at_end() {
            let expr = self.expression();
            res.push(expr);
        }
        res
    }
    


}
