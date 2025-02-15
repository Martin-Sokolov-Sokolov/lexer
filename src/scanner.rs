use core::panic;
use std::fmt::Display;
use std::fmt;
use std::process;

#[derive(Debug)]
enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,

    Star, Dot, Comma, Plus,

    EOF
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp = match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
            TokenType::Star => "STAR",
            TokenType::Dot => "DOT",
            TokenType::Comma => "COMMA",
            TokenType::Plus => "PLUS",

            TokenType::EOF => "EOF",
        };
        temp.fmt(f)
    }
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal,
            line
        }
    }

}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        format!("{} {} {}", self.token_type, self.lexeme, self.literal).fmt(f)
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    pub tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    pub fn get_tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '*' => self.add_token(TokenType::Star),
            ',' => self.add_token(TokenType::Comma),
            '+' => self.add_token(TokenType::Plus),
            '.' => self.add_token(TokenType::Dot),
            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
            }
        }
    }

    fn advance (&mut self) -> char {
        let temp = self.current;
        self.current += 1;
        self.source.chars().nth(temp).unwrap_or_else(|| 'a')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), String::from("null"), self.line));
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, String::from("null"));
    }

    fn add_token_helper(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, String::from(text), literal, self.line));
    }

}