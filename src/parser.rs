use crate::scanner::*;
use std::borrow::Cow;
use std::{fmt, process};
use std::process::exit;

#[derive(Debug)]
pub enum Expr {
    Lit(Literal),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Lit(Literal::False(b)) | Expr::Lit(Literal::True(b)) => write!(f, "{}", b),
            Expr::Lit(Literal::Nil) => write!(f, "nil"),
            Expr::Lit(Literal::Str(s)) => write!(f, "{}", unescape(s)),
            Expr::Lit(Literal::Number(n)) => write!(f, "{n:?}"),
            Expr::Binary(left, operator, right) => write!(f, "({} {} {})", operator, left, right),
            Expr::Unary(operator, right) => write!(f, "({} {})", operator, right),
            Expr::Grouping(expr) => write!(f, "(group {})", expr),
        }
    }
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
    Not,   
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnaryOp::Negate => write!(f, "-"),
            UnaryOp::Not => write!(f, "!"),
        }
    }
}

impl UnaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<UnaryOp> {
        match token_type {
            TokenType::Minus => Some(UnaryOp::Negate),
            TokenType::Bang => Some(UnaryOp::Not),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub enum BinaryOp {
    Equals,
    EqualEqual,
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

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let symbol = match self {
            BinaryOp::Equals => "=",
            BinaryOp::EqualEqual => "==",
            BinaryOp::NotEquals => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
        };
        write!(f, "{}", symbol)
    }
}

impl BinaryOp {
    pub fn from_token_type(token_type: &TokenType) -> Option<BinaryOp> {
        match token_type {
            TokenType::Equal => Some(BinaryOp::Equals),
            TokenType::EqualEqual => Some(BinaryOp::EqualEqual),
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
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0}
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.mat(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.mat(&[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.mat(&[TokenType::Minus, TokenType::Plus]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.mat(&[TokenType::Star, TokenType::Slash]) {
            let operator = BinaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.mat(&[TokenType::Minus, TokenType::Bang]) {
            let operator = UnaryOp::from_token_type(&self.previous().token_type).unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.mat(&[TokenType::False]) {
            return Ok(Expr::Lit(Literal::False(false)));
        }
        else if self.mat(&[TokenType::True]) {
            return Ok(Expr::Lit(Literal::True(true)));
        }
        else if self.mat(&[TokenType::Nil]) {
            return Ok(Expr::Lit(Literal::Nil));
        }
        else if self.mat(&[TokenType::String]) {
            if let Some(lit) = &self.previous().literal {
                if let Some(str_val) = lit.downcast_ref::<String>() {
                    return Ok(Expr::Lit(Literal::Str(str_val.to_string())));
                }
            }
        }
        else if self.mat(&[TokenType::LeftParen]) {
            let expr = self.expression()?;

            if self.consume(&TokenType::RightParen) {
                return Ok(Expr::Grouping(Box::from(expr)));
            }
            else {
                return Err(format!("change error"));
            }
        }
        
        let p = self.peek();
        if let TokenType::Number(n) = p.token_type {
            if self.mat(&[TokenType::Number(n)]) {
                if let Some(lit) = &self.previous().literal {
                    if let Some(num_val) = lit.downcast_ref::<f64>() {
                        return Ok(Expr::Lit(Literal::Number(*num_val)));
                    }
                }
            }
        }
        let a = self.peek();
        Err(format!("[line {}] Error at '{}': Expect expression.", a.line, a.lexeme))
    }

    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
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

    fn consume(&mut self, token_type: &TokenType) -> bool {
        if self.check(token_type) {
            self.advance();
            true
        }
        else {
            false
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

    fn mat(&mut self, v: &[TokenType]) -> bool {
        for token_type in v {
            if self.check(&token_type) {
                self.advance();
                return true;
            }
        }
        false
    }
}

pub fn unescape(s: &str) -> Cow<str> {
    Cow::Borrowed(s.trim_matches('"'))
}
