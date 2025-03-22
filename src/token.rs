use std::borrow::Cow;
use std::fmt;

use crate::expr::Literal;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    Equal,
    String(String),
    Identifier,
    Number(f64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    EOF
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<Literal>>,
    pub line: usize,
}

impl Token {
    pub fn unescape(s: & str) -> Cow<str> {
        Cow::Borrowed(s.trim_matches('"'))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.lexeme.clone();
        match &self.token_type {
            TokenType::LeftParen => write!(f, "LEFT_PAREN {origin} null"),
            TokenType::RightParen => write!(f, "RIGHT_PAREN {origin} null"),
            TokenType::LeftBrace => write!(f, "LEFT_BRACE {origin} null"),
            TokenType::RightBrace => write!(f, "RIGHT_BRACE {origin} null"),
            TokenType::Comma => write!(f, "COMMA {origin} null"),
            TokenType::Dot => write!(f, "DOT {origin} null"),
            TokenType::Minus => write!(f, "MINUS {origin} null"),
            TokenType::Plus => write!(f, "PLUS {origin} null"),
            TokenType::SemiColon => write!(f, "SEMICOLON {origin} null"),
            TokenType::Star => write!(f, "STAR {origin} null"),
            TokenType::BangEqual => write!(f, "BANG_EQUAL {origin} null"),
            TokenType::EqualEqual => write!(f, "EQUAL_EQUAL {origin} null"),
            TokenType::LessEqual => write!(f, "LESS_EQUAL {origin} null"),
            TokenType::GreaterEqual => write!(f, "GREATER_EQUAL {origin} null"),
            TokenType::Less => write!(f, "LESS {origin} null"),
            TokenType::Greater => write!(f, "GREATER {origin} null"),
            TokenType::Slash => write!(f, "SLASH {origin} null"),
            TokenType::Bang => write!(f, "BANG {origin} null"),
            TokenType::Equal => write!(f, "EQUAL {origin} null"),
            TokenType::String(_) => write!(f, "STRING {origin} {}", Token::unescape(&origin)),
            TokenType::Identifier => write!(f, "IDENTIFIER {origin} null"),
            TokenType::Number(n) => {
                if *n == n.trunc() {
                    write!(f, "NUMBER {origin} {n}.0")
                } else {
                    write!(f, "NUMBER {origin} {n}")
                }
            }
            TokenType::And => write!(f, "AND {origin} null"),
            TokenType::Class => write!(f, "CLASS {origin} null"),
            TokenType::Else => write!(f, "ELSE {origin} null"),
            TokenType::False => write!(f, "FALSE {origin} null"),
            TokenType::For => write!(f, "FOR {origin} null"),
            TokenType::Fun => write!(f, "FUN {origin} null"),
            TokenType::If => write!(f, "IF {origin} null"),
            TokenType::Nil => write!(f, "NIL {origin} null"),
            TokenType::Or => write!(f, "OR {origin} null"),
            TokenType::Print => write!(f, "PRINT {origin} null"),
            TokenType::Return => write!(f, "RETURN {origin} null"),
            TokenType::Super => write!(f, "SUPER {origin} null"),
            TokenType::This => write!(f, "THIS {origin} null"),
            TokenType::True => write!(f, "TRUE {origin} null"),
            TokenType::Var => write!(f, "VAR {origin} null"),
            TokenType::While => write!(f, "WHILE {origin} null"),
            TokenType::EOF => write!(f, "EOF  null"),
        }
    }
}