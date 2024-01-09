use crate::expr::Expr;
use crate::literal::Literal;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;
use tracing::debug;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn run(&mut self) -> Result<Vec<Stmt>> {
        let mut statements = Vec::new();

        while !self.is_at_end() {
            let s = self.declaration()?;
            debug!("{s}");
            statements.push(s);
        }

        Ok(statements)
    }

    // expression -> equality ;
    fn expression(&mut self) -> Result<Box<Expr>> {
        // self.equality()
        self.assignment()
    }

    // declaration -> var_decl | statement ;
    fn declaration(&mut self) -> Result<Stmt> {
        let re = if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match re {
            Ok(stmt) => Ok(stmt),
            Err(error) => {
                self.synchronize()?;
                bail!("{error}")
            }
        }
    }

    // statement -> expr_stmt | for_stmt | if_stmt | print_stmt | while_stmt | block ;
    fn statement(&mut self) -> Result<Stmt> {
        if self.is_match(&[TokenType::For]) {
            return self.for_statement();
        }
        if self.is_match(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.is_match(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.is_match(&[TokenType::While]) {
            return self.while_statement();
        }
        if self.is_match(&[TokenType::LeftBrace]) {
            return self.block();
        }
        self.expression_statement()
    }

    // for_stmt -> "for" "(" ( var_decl | expr_stmt | ";" ) expression? ";" expression? ")" statement ;
    fn for_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'for'.")?;

        let initializer = if self.is_match(&[TokenType::Semicolon]) {
            None
        } else if self.is_match(&[TokenType::Var]) {
            Some(self.var_declaration()?)
        } else {
            Some(self.expression_statement()?)
        };

        let mut condition = None;
        if !self.check(TokenType::Semicolon) {
            condition = Some(self.expression()?);
        }
        self.consume(TokenType::Semicolon, "Expect ';' after loop condition.")?;

        let mut increment = None;
        if !self.check(TokenType::RightParen) {
            increment = Some(self.expression()?);
        }
        self.consume(TokenType::RightParen, "Expect ')' after for clauses.")?;

        let mut body = self.statement()?;
        if let Some(i) = increment {
            body = Stmt::Block(Vec::from([body, Stmt::Expression(i)]));
        }
        if condition.is_none() {
            condition = Some(Box::new(Expr::Literal(Literal::Boolean(true))));
        }
        body = Stmt::While(condition.expect("Failed to get value"), Box::new(body));

        if let Some(i) = initializer {
            body = Stmt::Block(Vec::from([i, body]));
        }

        Ok(body)
    }

    // if_stmt -> "if" "(" expression ")" statement ( "else" statement )? ;
    fn if_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after if condition.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.is_match(&[TokenType::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };

        Ok(Stmt::If(condition, Box::new(then_branch), else_branch))
    }

    // print_stmt -> "print" expression ";" ;
    fn print_statement(&mut self) -> Result<Stmt> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after value.")?;
        Ok(Stmt::Print(value))
    }

    // var_decl -> "var" identifier ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();
        let initializer = if self.tokens[self.current].token_type == TokenType::Equal {
            self.advance();
            Some(self.expression()?)
        } else {
            None
        };
        self.consume(
            TokenType::Semicolon,
            "Expect ';' after variable declaration.",
        )?;
        Ok(Stmt::Var(name, initializer))
    }

    // while_stmt -> "while" "(" expression ")" statement ;
    fn while_statement(&mut self) -> Result<Stmt> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after condition.")?;
        let body = self.statement()?;

        Ok(Stmt::While(condition, Box::new(body)))
    }

    // expr_stmt -> expression ";" ;
    fn expression_statement(&mut self) -> Result<Stmt> {
        let expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(expr))
    }

    // block -> "{" declaration* "}" ;
    fn block(&mut self) -> Result<Stmt> {
        let mut statements = Vec::new();

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            statements.push(self.declaration()?);
        }

        self.consume(TokenType::RightBrace, "Expect '}}' after block.")?;
        Ok(Stmt::Block(statements))
    }

    // assignment -> identifier "=" assignment | logic_or ;
    fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match *expr {
                Expr::Variable(name) => return Ok(Box::new(Expr::Assign(name, value))),
                _ => bail!(report(&equals, "Invalid assignment target")),
            }
        }

        Ok(expr)
    }

    // logic_or -> logic_and ( "or" logic_and )* ;
    fn or(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.and()?;

        while self.is_match(&[TokenType::Or]) {
            let operator = self.previous().clone();
            let right = self.and()?;
            expr = Box::new(Expr::Logical(expr, operator, right));
        }

        Ok(expr)
    }

    // logic_and -> equality ( "and" equality )* ;
    fn and(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.equality()?;

        while self.is_match(&[TokenType::And]) {
            let operator = self.previous().clone();
            let right = self.equality()?;
            expr = Box::new(Expr::Logical(expr, operator, right));
        }

        Ok(expr)
    }

    // equality -> comparison ( ( "!=" | "==" ) comparison )* ;
    fn equality(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.comparison()?;

        let t = [TokenType::BangEqual, TokenType::EqualEqual];
        while self.is_match(&t) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;

        let t = [
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ];
        while self.is_match(&t) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;

        let t = [TokenType::Minus, TokenType::Plus];
        while self.is_match(&t) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // factor -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;

        let t = [TokenType::Slash, TokenType::Star];
        while self.is_match(&t) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Box<Expr>> {
        match self.peek().token_type {
            TokenType::Bang | TokenType::Minus => {
                self.advance();
                let operator = self.previous().clone();
                let right = self.unary()?;
                Ok(Box::new(Expr::Unary(operator.clone(), right.clone())))
            }
            _ => Ok(Box::new(self.primary()?)),
        }
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | identifier ;
    fn primary(&mut self) -> Result<Expr> {
        match self.peek().token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => Ok(Expr::Literal(self.advance().literal.clone())),
            TokenType::Identifier => Ok(Expr::Variable(self.advance().clone())),
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
                Ok(Expr::Grouping(expr))
            }
            _ => bail!(report(self.peek(), "Expect expression.")),
        }
    }

    fn synchronize(&mut self) -> Result<()> {
        let t = [
            TokenType::Class,
            TokenType::Fun,
            TokenType::Var,
            TokenType::For,
            TokenType::If,
            TokenType::While,
            TokenType::Print,
            TokenType::Return,
        ];

        self.advance();
        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon {
                return Ok(());
            }
            if t.contains(&self.peek().token_type) {
                return Ok(());
            }
            self.advance();
        }

        Ok(())
    }

    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<&Token> {
        if self.peek().token_type == token_type {
            return Ok(self.advance());
        }
        bail!(report(self.peek(), message))
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.contains(&self.peek().token_type) && !self.is_at_end() {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, token_type: TokenType) -> bool {
        self.peek().token_type == token_type
    }

    // Consume the current token and return it.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    // Return the current token not yet consumed.
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    // Return the last token consumed.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }

    // Check if the token list has been parsed to the end.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }
}

fn report(token: &Token, message: &str) -> String {
    let place = match token.token_type {
        TokenType::Eof => " at end".to_string(),
        _ => format!(" at '{}'", token.lexeme),
    };
    format!("[line {}] Error{}: {}", token.line, place, message)
}
