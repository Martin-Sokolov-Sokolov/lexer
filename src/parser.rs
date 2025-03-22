use crate::{expr::{BinaryOp, Expr, Literal, UnaryOp}, stmt::Stmt, token::{Token, TokenType}};

pub struct Parser <'a> {
    tokens: &'a Vec<Token>,
    current: usize
}

impl <'a> Parser <'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self { 
            tokens,
            current: 0
        }
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.comparison()?;

        while self.mat(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous()?.token_type).unwrap();
            let right = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.term()?;
        while self.mat(&[TokenType::Less, TokenType::LessEqual, TokenType::Greater, TokenType::GreaterEqual]) {
            let operator = BinaryOp::from_token_type(&self.previous()?.token_type).unwrap();
            let right = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, String> {
        let mut expr = self.factor()?;
        while self.mat(&[TokenType::Minus, TokenType::Plus]) {
            let operator = BinaryOp::from_token_type(&self.previous()?.token_type).unwrap();
            let right = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.unary()?;
        while self.mat(&[TokenType::Star, TokenType::Slash]) {
            let operator = BinaryOp::from_token_type(&self.previous()?.token_type).unwrap();
            let right = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if self.mat(&[TokenType::Minus, TokenType::Bang]) {
            let operator = UnaryOp::from_token_type(&self.previous()?.token_type).unwrap();
            let right = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }
        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, String> {
        if self.mat(&[TokenType::False]) {
            return Ok(Expr::Lit(Literal::Boolean(false)));
        }
        else if self.mat(&[TokenType::True]) {
            return Ok(Expr::Lit(Literal::Boolean(true)));
        }
        else if self.mat(&[TokenType::Nil]) {
            return Ok(Expr::Lit(Literal::Nil));
        }
        else if self.mat(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(&TokenType::RightParen, "Expect ')' after expression.".to_string())?;
            return Ok(Expr::Grouping(Box::from(expr)));
        }

        let token_type = &self.peek().token_type;
        if let TokenType::Number(n) = token_type {
            if self.mat(&[TokenType::Number(*n)]) {
                if let Some(Literal::Number(num_val)) = self.previous()?.literal.as_deref() {
                    return Ok(Expr::Lit(Literal::Number(*num_val)));
                }
            }
        }
        else if let TokenType::String(s) = token_type {
            if self.mat(&[TokenType::String(s.to_string())]) {
                if let Some(Literal::Str(str_val)) = &self.previous()?.literal.as_deref() {
                    return Ok(Expr::Lit(Literal::Str(String::from(str_val))));
                }
            }
        }
        else if self.mat(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous()?.lexeme.to_string()));
        }

        let a = self.peek();
        Err(format!("[line {}] Error at '{}': Expect expression.", a.line, a.lexeme))
    }


    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    fn advance(&mut self) -> Result<&Token, String> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn consume(&mut self, token_type: &TokenType, err_message: String) -> Result<&Token, String> {
        if self.check(token_type) {
            self.advance()
        }
        else {
            Err(err_message)
        }
    }

    fn previous(&self) -> Result<&Token, String> {
        Ok(self.tokens.get(self.current-1).unwrap())
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
                if let Ok(_) = self.advance() {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
        false
    }
    
    pub fn parse(&mut self) -> Result<Expr, String> {
        self.expression()
    }

    pub fn _parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts: Vec<Stmt> = Vec::new();

        while !self.is_at_end() { 
            stmts.push(self.declaration()?);
        }

        return Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.mat(&[TokenType::Var]) {
            return self.var_declaration();
        }
        if self.mat(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Box::from(self.block()?)));
        }

        self.statement().or_else(|err| {
            self.synchronize()?;
            Err(err)
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let name = self.consume(&TokenType::Identifier, "Expect variable name.".to_string())?.lexeme.clone();
        let mut initializer: Option<Box<Expr>> = None;
        if self.mat(&[TokenType::Equal]) {
            initializer = Some(Box::from(self.expression()?));
        }
        self.consume(&TokenType::SemiColon, "Expect ';' after variable declaration.".to_string())?;
        
        return Ok(Stmt::Declaration { 
            id: name.to_string(), initializer: initializer, 
        });
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.mat(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.mat(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.mat(&[TokenType::LeftBrace]) {
            return Ok(Stmt::Block(Box::from(self.block()?)))
        }
        if self.mat(&[TokenType::While]) {
            return self.fn_while();
        }
        return self.expression_statement();
    }

    fn fn_while(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenType::LeftParen, "Expect '(' after 'while'".to_string())?;
        let expr = Box::from(self.expression()?);
        self.consume(&TokenType::RightParen, "Expect ')' after condition.".to_string())?;

        let statement = Box::from(self.statement()?);

        Ok(Stmt::While(expr, statement))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();

        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(&TokenType::RightBrace, "Expect '}' after block.".to_string())?;
        Ok(statements)
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expect ';' after value.".to_string())?;
        return Ok(Stmt::PrintStmt(Box::from(expr)));
    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenType::LeftParen, "Expected '(' before expression.".to_string())?;
        let expr = Box::from(self.expression()?);
        self.consume(&TokenType::RightParen, "Expected ')' after expression".to_string())?;
        
        let if_stmt = Box::from(self.statement()?);

        let mut else_val: Option<Box<Stmt>> = None;
        if self.mat(&[TokenType::Else]) {
            else_val = Some(Box::from(self.statement()?));
        }

        Ok(Stmt::If(expr, if_stmt, else_val))
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr = self.expression()?;
        self.consume(&TokenType::SemiColon, "Expected ';' after expression.".to_string())?;
        return Ok(Stmt::ExprStmt(Box::from(expr)));
    }
    
    fn assignment(&mut self) -> Result<Expr, String> {
        let expr = self.f_or()?;

        if self.mat(&[TokenType::Equal]) {
            let _ = self.previous()?;
            let val = self.assignment()?;

            match expr {
                Expr::Variable(t) => {
                    return Ok(Expr::Assign(t, Box::from(val)));
                },
                _ => return Err("Invalid assignment target.".to_string()),
            }
        }

        Ok(expr)
    }

    fn f_or(&mut self) -> Result<Expr, String> {
        let mut expr = self.f_and()?;

        while self.mat(&[TokenType::Or]) {
            let tok = self.previous()?.clone();
            let right = self.f_and()?;
            expr = Expr::Logical(Box::from(expr), Box::from(tok), Box::from(right));
        }

        return Ok(expr);
    }

    fn f_and(&mut self) -> Result<Expr, String> {
        let mut expr = self.equality()?;

        while self.mat(&[TokenType::And]) {
            let tok = self.previous()?.clone();
            let right = self.f_and()?;
            expr = Expr::Logical(Box::from(expr), Box::from(tok), Box::from(right))
        }

        return Ok(expr);
    }

    fn synchronize(&mut self) -> Result<(), String> {
        self.advance()?;
        while !self.is_at_end() {

            if self.previous()?.token_type == TokenType::SemiColon {
                return Ok(());
            }
            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Var | TokenType::For |
                TokenType::If | TokenType::While | TokenType::Print | TokenType::Return => {
                    return Ok(());
                }
                _ => {}
            }
            self.advance()?;
        
        }
        Ok(())
    }

}