use crate::expr::Expr;
use crate::{
    token::{Literal, Token},
    token_type::TokenType,
};
use anyhow::{bail, Result};

#[allow(dead_code)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    fn new() -> Self {
        Parser {
            tokens: Vec::new(),
            current: 0,
        }
    }

    fn expression(&mut self) -> Result<()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<()> {
        self.comparison()
    }

    fn comparison(&mut self) -> Result<()> {
        Ok(())
    }

    fn term(&mut self) -> Result<()> {
        Ok(())
    }

    fn factor(&mut self) -> Result<()> {
        Ok(())
    }

    fn unary(&mut self) -> Result<()> {
        Ok(())
    }

    fn primary(&mut self) -> Result<Expr> {
        if self.check_token_type(&[TokenType::False]) {
            return Ok(Expr::Literal(Token::new(
                TokenType::False,
                "false",
                Literal::None,
                1,
            )));
        } else if self.check_token_type(&[TokenType::True]) {
            return Ok(Expr::Literal(Token::new(
                TokenType::True,
                "true",
                Literal::None,
                1,
            )));
        } else if self.check_token_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Token::new(
                TokenType::False,
                "null",
                Literal::None,
                1,
            )));
        } else if self.check_token_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous()?.clone()));
        }

        Ok(Expr::Literal(Token::new(
            TokenType::And,
            "and",
            Literal::None,
            1,
        )))
    }

    fn check_token_type(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.is_type_match(token_type) {
                self.advance().unwrap();
                return true;
            }
        }
        false
    }

    fn is_type_match(&mut self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().unwrap().token_type == token_type
    }

    fn advance(&mut self) -> Result<&Token> {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().unwrap().token_type == TokenType::Eof
    }

    fn peek(&mut self) -> Result<&Token> {
        match self.tokens.get(self.current) {
            Some(t) => Ok(t),
            None => bail!("Failed to get a current token"),
        }
    }

    fn previous(&mut self) -> Result<&Token> {
        match self.tokens.get(self.current - 1) {
            Some(t) => Ok(t),
            None => bail!("Failed to get a previous token"),
        }
    }
}
