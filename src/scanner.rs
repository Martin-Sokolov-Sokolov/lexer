use std::any::Any;
use crate::token::{Token, TokenType};


#[derive(Debug)]
pub struct Scanner <'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
    pub code:i32,
}

impl <'a> Scanner <'a> {
    pub fn new(source: &'a str) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            code: 0,
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token_alternative();
        }
        self.tokens.push(Token { token_type: TokenType::EOF, lexeme: "null".to_string(), literal: None, line: 1 });

        &self.tokens
    }

    fn scan_token_alternative(&mut self) {
        let c = self.advance();

        let _ = match c {

            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            '*' => self.add_token(TokenType::Star),
            ',' => self.add_token(TokenType::Comma),
            '+' => self.add_token(TokenType::Plus),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::SemiColon),
            '!' => {
                let token_type = if !self.match_next('=') {TokenType::Bang} else {TokenType::BangEqual};
                self.add_token(token_type)
            }
            '=' => {
                let token_type = if !self.match_next('=') {TokenType::Equal} else {TokenType::EqualEqual};
                self.add_token(token_type)
            }
            '<' => {
                let token_type = if !self.match_next('=') {TokenType::Less} else {TokenType::LessEqual};
                self.add_token(token_type)

            }
            '>' => {
                let token_type = if !self.match_next('=') {TokenType::Greater} else {TokenType::GreaterEqual};
                self.add_token(token_type)
            }
            '/' => {
                if let Some(token_type) = self.slash() {
                    self.add_token(token_type)
                }
            }
            '\n' => self.line += 1,
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '"' => self.make_string_alternative(),
            '0'..='9' => self.number_alternative(),
            'a'..='z' | 'A'..='Z' | '_' => self.make_identifier_alternative(),
            '\0' => (),

            _ => {
                eprintln!("[line {}] Error: Unexpected character: {}", self.line, c);
                println!("{}", c.to_ascii_lowercase() as u8);
                self.code = 65;
            }
        };
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap() != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance (&mut self) -> char {
        let temp = self.current.clone();
        self.current += 1;
        self.source.chars().nth(temp).unwrap_or_else(|| '\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len() 
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or_else(|| '\0')
    }

    fn slash(&mut self) -> Option<TokenType> {
        if self.match_next('/') {
            while !self.is_at_end() && self.peek() != '\n' {
                self.advance();
            }
            None
        }
        else {
            Some(TokenType::Slash)
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, Some(Box::new(String::from("null"))));
    }

    fn add_token_helper(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text: &String = &self.source.chars().take(self.current).skip(self.start).collect();
        self.tokens.push(Token {token_type, lexeme: String::from(text), literal, line:self.line});
    }

    fn make_string_alternative(&mut self) {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }
        if self.is_at_end() {
            self.code = 65;
            eprintln!("[line {}] Error: Unterminated string.", self.line);
        } 
        else {
            self.advance();
    
            let text: String = self.source.chars().take(self.current).skip(self.start).collect();
            let lit = text.replace('"', "");
    
            self.add_token_helper(TokenType::String(text), Some(Box::new(lit)));
        }
    }
    
    fn peek_next(&self) -> char {
        return self.source.chars().nth(self.current+1).unwrap();
    }

    fn number_alternative(&mut self) {
        while !self.is_at_end() && is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' {
            if is_digit(self.peek_next()) {
                self.advance();

                while !self.is_at_end() && is_digit(self.peek()) {
                    self.advance();
                }
            }
        }

        let num_str = &self.source[self.start..self.current];
        let n = normalize_number_string(num_str);
        self.add_token_helper(TokenType::Number(n), Some(Box::new(n)));
    }

    fn make_identifier_alternative(&mut self) {
        while !self.is_at_end() && is_alpha_numric(self.peek()) {
            self.advance();
        }
        let ident = &self.source[self.start..self.current];
        let kind = match ident {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifier,
        };
        self.add_token(kind);

    }

}

fn normalize_number_string(num_str: &str) -> f64 {
    match num_str.parse::<f64>() {
        Ok(num) => {
            num
        }
        Err(_) => 0.0
    }
}

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    return c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_';
}

fn is_alpha_numric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}