use std::fmt::Display;
use std::{default, fmt};
use std::thread::current;

enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let temp = match self {
            TokenType::LeftParen => "LEFT_PAREN",
            TokenType::RightParen => "RIGHT_PAREN",
            TokenType::LeftBrace => "LEFT_BRACE",
            TokenType::RightBrace => "RIGHT_BRACE",
        };
        temp.fmt(f)
    }
}

struct Token {
    source: String,
    tokens: Vec<TokenType>,
    start: usize,
    current: usize,
    line: usize,
}

impl Token {
    pub fn new(&self, source: String) -> Self {
        Token {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1
        }
    }

    fn scan_token(&mut self) -> TokenType {
        let c = self.advance();

        match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightBrace,
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            _ => panic!("No such character found in dictionary!")
        }
    }

    fn advance (&mut self) -> char {
        let temp = self.current;
        self.current += 1;
        self.source.chars().nth(temp).unwrap()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn scan_tokens(&mut self) -> &str {
        while(!self.is_at_end()) {
            let token_type = self.scan_token();
        }

        "asd"
    } 

}