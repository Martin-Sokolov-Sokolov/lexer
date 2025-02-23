use std::any::Any;
use std::borrow::Cow;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    String,
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
    Empty,
    EOF
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
}

impl Token {
    pub fn is_empty(&self) -> bool {
        return self.token_type == TokenType::Empty;
    }

    pub fn unescape(s: & str) -> Cow<str> {
        Cow::Borrowed(s.trim_matches('"'))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let origin = self.lexeme.clone();
        match self.token_type {
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
            TokenType::String => write!(f, "STRING {origin} {}", Token::unescape(&origin)),
            TokenType::Identifier => write!(f, "IDENTIFIER {origin} null"),
            TokenType::Number(n) => {
                if n == n.trunc() {
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
            TokenType::Empty => write!(f, ""),
            TokenType::EOF => write!(f, "EOF  null"),
        }
    }
}

#[derive(Debug)]
pub struct Scanner {
    source: String,
    start: usize,
    current: usize,
    line: usize,
}

impl Iterator for Scanner {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        while !self.is_at_end() {
            self.start = self.current;
            let res = self.scan_token_alternative();

            match res {
                Ok(tok) if tok.token_type == TokenType::Empty => continue,
                _ => return Some(res),
            }
        }
        None
    }
}



impl Scanner {
    pub fn new(source: String) -> Self {
        Scanner {
            source,
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_token_alternative(&mut self) -> Result<Token, String> {
        let c = self.advance();

        let res = match c {

            '(' => self.return_token(TokenType::LeftParen),
            ')' => self.return_token(TokenType::RightParen),
            '{' => self.return_token(TokenType::LeftBrace),
            '}' => self.return_token(TokenType::RightBrace),
            '*' => self.return_token(TokenType::Star),
            ',' => self.return_token(TokenType::Comma),
            '+' => self.return_token(TokenType::Plus),
            '.' => self.return_token(TokenType::Dot),
            '-' => self.return_token(TokenType::Minus),
            ';' => self.return_token(TokenType::SemiColon),
            '!' => {
                let token_type = if !self.match_next('=') {TokenType::Bang} else {TokenType::BangEqual};
                self.return_token(token_type)
            }
            '=' => {
                let token_type = if !self.match_next('=') {TokenType::Equal} else {TokenType::EqualEqual};
                self.return_token(token_type)
            }
            '<' => {
                let token_type = if !self.match_next('=') {TokenType::Less} else {TokenType::LessEqual};
                self.return_token(token_type)

            }
            '>' => {
                let token_type = if !self.match_next('=') {TokenType::Greater} else {TokenType::GreaterEqual};
                self.return_token(token_type)
            }
            '/' => {
                if let Some(token_type) = self.slash() {
                    self.return_token(token_type)
                }
                else {
                    self.return_token(TokenType::Empty)
                }
            }
            '\n' => {
                self.line += 1;
                self.return_token(TokenType::Empty)
            },
            ' ' => self.return_token(TokenType::Empty),
            '\r' => self.return_token(TokenType::Empty),
            '\t' => self.return_token(TokenType::Empty),
            '"' => {
                if let Ok(token) = self.make_string_alternative() {
                    token
                }
                else {
                    return Err(format!("[line {}] Error: Unterminated string.", self.line));
                }
            }
            '0'..='9' => {
                if let Ok(token) = self.number_alternative() {
                    token
                }
                else {
                    return Err(String::from("number error"));
                }
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                if let Ok(token) = self.make_identifier_alternative() {
                    token
                }
                else {
                    return Err(String::from("ident error"));
                }
            }

            _ => return Err(format!("[line {}]: Error: Unexpected character: {}", self.line, c)),
        };

        Ok(res)
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

    fn return_token(&self, token_type: TokenType) -> Token {
        self.return_token_helper(token_type, Some(Box::new(String::from("null"))))
    }

    fn return_token_helper(&self, token_type: TokenType, literal: Option<Box<dyn Any>>) -> Token {
        let text = &self.source[self.start..self.current];
        Token {token_type, lexeme: String::from(text), literal}
    }

    fn make_string_alternative(&mut self) -> Result<Token, String> {
        while !self.is_at_end() && self.peek() != '"' {
            self.advance();
        }

        if self.is_at_end() {
            Err(format!("[line {}] Error: Unterminated string.", self.line))
        }
        else {
            self.advance();
            let text = &self.source[self.start..self.current];
            let lit = text.replace('"', "");
            return Ok(Token{token_type: TokenType::String, lexeme: String::from(text), literal: Some(Box::new(lit))});
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        else {
            return self.source.chars().nth(self.current+1).unwrap();
        }
    }

    fn number_alternative(&mut self) -> Result<Token, ()> {
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
        return Ok(self.return_token_helper(TokenType::Number(n), Some(Box::new(n))));
    }

    fn make_identifier_alternative(&mut self) -> Result<Token, ()> {
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
        Ok(self.return_token(kind))

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