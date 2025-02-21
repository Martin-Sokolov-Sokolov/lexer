use std::fmt::Display;
use std::fmt;


#[derive(Debug, PartialEq)]
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
    Number,
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
    Empty
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
            TokenType::Number => "NUMBER",
            TokenType::Identifier => "IDENTIFIER",
            TokenType::Fun => "FUN",
            TokenType::And => "AND",
            TokenType::Class => "CLASS",
            TokenType::Else => "ELSE",
            TokenType::False => "FALSE",
            TokenType::For => "FOR",
            TokenType::If => "IF",
            TokenType::Nil => "NIL",
            TokenType::Or => "OR",
            TokenType::Print => "PRINT",
            TokenType::Return => "RETURN",
            TokenType::Super => "SUPER",
            TokenType::This => "THIS",
            TokenType::True => "TRUE",
            TokenType::Var => "VAR",
            TokenType::While => "WHILE",
            TokenType::Empty => "",
        };
        temp.fmt(f)
    }
}
#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String
}

impl Token {
    pub fn is_empty(&self) -> bool {
        return self.token_type == TokenType::Empty;
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
    start: usize,
    current: usize,
    line: usize,
}

impl Iterator for Scanner {
    type Item = Result<Token, String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.start = self.current;
        if self.is_at_end() {
            return None;
        }
        else {
            let res = self.scan_token_alternative();

            match res {
                Ok(token) => Some(Ok(token)),
                Err(err) => Some(Err(err)),
            }

        }
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
                    return Err(format!("[line {}] Error: Unterminated string", self.line));
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
        self.return_token_helper(token_type, String::from("null"))
    }

    fn return_token_helper(&self, token_type: TokenType, literal: String) -> Token {
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
            return Ok(Token{token_type: TokenType::String, lexeme: String::from(text), literal: lit});
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
        let num = normalize_number_string(num_str);
        return Ok(self.return_token_helper(TokenType::Number, num));
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

fn normalize_number_string(num_str: &str) -> String {
    match num_str.parse::<f64>() {
        Ok(num) => {
            if num.fract() == 0.0 {
                format!("{:.1}", num)
            } else {
                num.to_string()
            }
        }
        Err(_) => num_str.to_string(),
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