use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;
use core::fmt;
use tracing::debug;

#[derive(Debug)]
enum ParseError {
    MissingRightParen,
    MissingExpression,
}

impl ParseError {
    fn report(&self, token: &Token) -> String {
        let place = match token.token_type {
            TokenType::Eof => " at end".to_string(),
            _ => format!(" at '{}'", token.lexeme),
        };
        format!("[line {}] Error{}: {}", token.line, place, self)
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingRightParen => write!(f, "Expect ')' after expression"),
            Self::MissingExpression => write!(f, "Expect expression"),
        }
    }
}

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn run(&mut self) -> Result<Expr> {
        match self.expression() {
            Ok(expr) => {
                debug!("{expr}");
                Ok(*expr)
            }
            Err(error) => bail!("{error}"),
        }
    }

    /// Rule: equality ;
    fn expression(&mut self) -> Result<Box<Expr>> {
        self.equality()
    }

    /// Rule: comparison ( ( "!=" | "==" ) comparison )* ;
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

    /// Rule: comparison -> term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
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

    /// Rule: term -> factor ( ( "-" | "+" ) factor )* ;
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

    /// Rule: factor -> unary ( ( "/" | "*" ) unary )* ;
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

    /// Rule: unary -> ( "!" | "-" ) unary | primary ;
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

    /// Rule: primary -> NUMBER | STRING | "true" | "false" | "nil" | "(" expression ")" ;
    fn primary(&mut self) -> Result<Expr> {
        match self.peek().token_type {
            TokenType::Number
            | TokenType::String
            | TokenType::True
            | TokenType::False
            | TokenType::Nil => {
                self.advance();
                Ok(Expr::Literal(self.previous().literal.clone()))
            }
            TokenType::LeftParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(TokenType::RightParen, ParseError::MissingRightParen)?;
                Ok(Expr::Grouping(expr))
            }
            _ => bail!(ParseError::MissingExpression.report(self.peek())),
        }
    }

    fn is_match(&mut self, token_types: &[TokenType]) -> bool {
        if token_types.contains(&self.peek().token_type) && !self.is_at_end() {
            self.advance();
            return true;
        }
        false
    }

    fn consume(&mut self, token_type: TokenType, message: ParseError) -> Result<&Token> {
        if self.check(&token_type) {
            return Ok(self.advance());
        }

        bail!(message.report(self.peek()))
    }

    fn check(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().token_type == token_type
    }

    /// Consume the current token and return it.
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Check if the token list has been parsed to the end.
    fn is_at_end(&mut self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Return the current token not yet consumed.
    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    /// Return the last token consumed.
    fn previous(&mut self) -> &Token {
        &self.tokens[self.current - 1]
    }

    fn _synchronize(&mut self) -> Result<()> {
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
}
