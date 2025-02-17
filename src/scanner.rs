use std::fmt::Display;
use std::fmt;


#[derive(Debug)]
enum TokenType {
    LeftParen, RightParen, LeftBrace, RightBrace,

    Star, Dot, Comma, Plus, Minus,
    Bang, BangEqual, Equal, EqualEqual, Less, LessEqual, Greater, GreaterEqual,

    Slash,

    SemiColon,

    String,

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
            TokenType::Minus => "MINUS",
            TokenType::SemiColon => "SEMICOLON",
            TokenType::Bang => "BANG",
            TokenType::BangEqual => "BANG_EQUAL",
            TokenType::Equal => "EQUAL",
            TokenType::EqualEqual => "EQUAL_EQUAL",
            TokenType::Less => "LESS",
            TokenType::LessEqual => "LESS_EQUAL",
            TokenType::Greater => "GREATER",
            TokenType::GreaterEqual => "GREATER_EQUAL",
            TokenType::Slash => "SLASH",
            TokenType::String => "STRING",
            
            TokenType::EOF => "EOF",
        };
        temp.fmt(f)
    }
}
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String
}

impl Token {
    fn new(token_type: TokenType, lexeme: String, literal: String) -> Self {
        Token {
            token_type,
            lexeme,
            literal
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
    errors: Vec<String>,
    pub code: i32,
}

impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            errors: Vec::new(),
            code: 0,
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
            '-' => self.add_token(TokenType::Minus),
            ';' => self.add_token(TokenType::SemiColon),
            '!' => {
                let token_type = if !self.match_next('=') {TokenType::Bang} else {TokenType::BangEqual};
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if !self.match_next('=') {TokenType::Equal} else {TokenType::EqualEqual};
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if !self.match_next('=') {TokenType::Less} else {TokenType::LessEqual};
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if !self.match_next('=') {TokenType::Greater} else {TokenType::GreaterEqual};
                self.add_token(token_type);
            }
            '/' => {
                if self.match_next('/') {
                    while !self.is_at_end() && self.peek() != '\n' {
                        self.advance();
                    }
                }
                else {
                    self.add_token(TokenType::Slash);
                }
            }
            '\n' => self.line += 1,
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '"' => {
                if let Ok(result) = self.make_string() {
                    self.add_token_helper(TokenType::String, result);
                }
                else {
                    self.errors.push(format!("[line {}] Error: Unterminated string.", self.line));
                }
            },
            _ => {
                self.errors.push(format!("[line {}] Error: Unexpected character: {}", self.line, c));
            }

        }
    }

    fn match_next(&mut self, c: char) -> bool {
        if self.is_at_end() || self.source.chars().nth(self.current).unwrap_or_else(|| return '\0') != c {
            return false;
        }
        self.current += 1;
        return true;
    }

    fn advance (&mut self) -> char {
        let temp = self.current;
        self.current += 1;
        self.source.chars().nth(temp).unwrap_or_else(|| return '\0')
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn peek(&self) -> char {
        self.source.chars().nth(self.current).unwrap_or_else(|| return '\0')
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(TokenType::EOF, String::from(""), String::from("null")));

        if !self.errors.is_empty() {
            for err in &self.errors {
                eprintln!("{}", err);
            }
            self.code = 65;
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_helper(token_type, String::from("null"));
    }

    fn add_token_helper(&mut self, token_type: TokenType, literal: String) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token::new(token_type, String::from(text), literal));
    }

    fn make_string(&mut self) -> Result<String, ()> {
        let mut res = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            let c = self.advance();
            res.push(c);
        }

        if self.is_at_end() {
            return Err(())
        }
        else {
            self.advance();
            Ok(res)
        }
    }

}