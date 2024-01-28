use crate::expr::Expr;
use crate::result::Error;
use crate::result::Result;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
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
        self.assignment()
    }

    // declaration -> fun_decl | var_decl | statement ;
    fn declaration(&mut self) -> Result<Stmt> {
        let result = if self.is_match(&[TokenType::Fun]) {
            self.function("function")
        } else if self.is_match(&[TokenType::Var]) {
            self.var_declaration()
        } else {
            self.statement()
        };
        match result {
            Ok(stmt) => Ok(stmt),
            Err(error) => {
                // TODO: Return null???
                self.synchronize()?;
                Err(error)
            }
        }
    }

    // statement -> expr_stmt | for_stmt | if_stmt | print_stmt | return_stmt | while_stmt | block ;
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
        if self.is_match(&[TokenType::Return]) {
            return self.return_statement();
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

        // Desugar increment
        if let Some(i) = increment {
            body = Stmt::Block(Vec::from([body, Stmt::Expression(i)]));
        }

        // Desugar condition
        if condition.is_none() {
            condition = Some(Box::new(Expr::Literal(Value::Boolean(true))));
        }
        body = Stmt::While(condition.expect("Failed to get value"), Box::new(body));

        // Desugar initializer
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

    // return_stmt -> "return" expression? ";" ;
    fn return_statement(&mut self) -> Result<Stmt> {
        let keyword = self.previous().clone();
        let value = if !self.check(TokenType::Semicolon) {
            Some(*self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expect ';' after return value.")?;
        Ok(Stmt::Return(keyword, value))
    }

    // var_decl -> "var" IDENTIFIER ( "=" expression )? ";" ;
    fn var_declaration(&mut self) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, "Expect variable name.")?
            .clone();

        let initializer = if self.is_match(&[TokenType::Equal]) {
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

    // function -> IDENTIFIER "(" parameters? ")" block ;
    fn function(&mut self, kind: &str) -> Result<Stmt> {
        let name = self
            .consume(TokenType::Identifier, &format!("Expect {} name.", kind))?
            .clone();

        self.consume(
            TokenType::LeftParen,
            &format!("Expect '(' after {} name.", kind),
        )?;
        let mut parameters = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if parameters.len() >= 255 {
                    return Err(Error::Parse(
                        self.peek().clone(),
                        "Can't have more than 255 parameters.".to_string(),
                    ));
                }

                let p = self
                    .consume(TokenType::Identifier, "Expect parameter name.")?
                    .clone();
                parameters.push(p);

                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }
        self.consume(TokenType::RightParen, "Expect ')' after parameters.")?;

        self.consume(
            TokenType::LeftBrace,
            &format!("Expect '{{' before {} body.", kind),
        )?;
        let body = self.block()?;
        Ok(Stmt::Function(name, parameters, Box::new(body)))
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

    // assignment -> IDENTIFIER "=" assignment | logic_or ;
    fn assignment(&mut self) -> Result<Box<Expr>> {
        let expr = self.or()?;

        if self.is_match(&[TokenType::Equal]) {
            let equals = self.previous().clone();
            let value = self.assignment()?;

            match *expr {
                Expr::Variable(name) => return Ok(Box::new(Expr::Assign(name, value))),
                _ => {
                    return Err(Error::Parse(
                        equals,
                        "Invalid assignment target".to_string(),
                    ))
                }
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

        while self.is_match(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous().clone();
            let right = self.comparison()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
    fn comparison(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.term()?;

        while self.is_match(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous().clone();
            let right = self.term()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // term -> factor ( ( "-" | "+" ) factor )* ;
    fn term(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.factor()?;

        while self.is_match(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous().clone();
            let right = self.factor()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // factor -> unary ( ( "/" | "*" ) unary )* ;
    fn factor(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.unary()?;

        while self.is_match(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            expr = Box::new(Expr::Binary(expr, operator, right));
        }

        Ok(expr)
    }

    // unary -> ( "!" | "-" ) unary | primary ;
    fn unary(&mut self) -> Result<Box<Expr>> {
        if self.is_match(&[TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous().clone();
            let right = self.unary()?;
            return Ok(Box::new(Expr::Unary(operator, right)));
        }

        self.call()
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr> {
        let mut arguments = Vec::new();
        if !self.check(TokenType::RightParen) {
            loop {
                if arguments.len() >= 255 {
                    return Err(Error::Parse(
                        self.peek().clone(),
                        "Can't have more than 255 arguments.".to_string(),
                    ));
                }

                arguments.push(*self.expression()?);
                if !self.is_match(&[TokenType::Comma]) {
                    break;
                }
            }
        }

        let paren = self.consume(TokenType::RightParen, "Expect ')' after arguments.")?;
        Ok(Expr::Call(Box::new(callee), paren.clone(), arguments))
    }

    // call -> primary ( "(" arguments? ")" )* ;
    // arguments -> expression ( "," expression )* ;
    fn call(&mut self) -> Result<Box<Expr>> {
        let mut expr = self.primary()?;

        loop {
            if self.is_match(&[TokenType::LeftParen]) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(Box::new(expr))
    }

    // primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" | IDENTIFIER ;
    fn primary(&mut self) -> Result<Expr> {
        if self.is_match(&[TokenType::Number, TokenType::String, TokenType::Nil]) {
            return Ok(Expr::Literal(self.previous().literal.clone().into()));
        }

        if self.is_match(&[TokenType::False]) {
            return Ok(Expr::Literal(Value::Boolean(false)));
        }
        if self.is_match(&[TokenType::True]) {
            return Ok(Expr::Literal(Value::Boolean(true)));
        }

        if self.is_match(&[TokenType::Identifier]) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        if self.is_match(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression.")?;
            return Ok(Expr::Grouping(expr));
        }

        Err(Error::Parse(
            self.peek().clone(),
            "Expect expression.".to_string(),
        ))
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
        Err(Error::Parse(self.peek().clone(), message.to_string()))
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
