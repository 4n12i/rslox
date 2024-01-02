use crate::error::get_parse_error;
use crate::expr::Expr;
use crate::literal::Literal;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

#[allow(dead_code)]
impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn run(&mut self) -> Result<Expr> {
        match self.expression() {
            Ok(e) => Ok(e),
            Err(_) => Ok(Expr::None),
        }
    }

    fn expression(&mut self) -> Result<Expr> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr> {
        self.comparison()
    }

    fn comparison(&mut self) -> Result<Expr> {
        Ok(Expr::Literal(Literal::None))
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
            return Ok(Expr::Literal(Literal::Bool(false)));
        } else if self.check_token_type(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::Bool(true)));
        } else if self.check_token_type(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::None));
        } else if self.check_token_type(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(self.previous()?.literal.clone()));
        } else if self.check_token_type(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            // TODO: consume()
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        bail!("Expect expression")
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

    fn consume(&mut self, token_type: &TokenType, message: &str) -> Result<&Token> {
        if self.is_type_match(token_type) {
            return self.advance();
        }

        bail!("{}", get_parse_error(self.peek()?, message)?)
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

    fn peek(&mut self) -> Result<Token> {
        match self.tokens.get(self.current) {
            Some(t) => Ok(t.to_owned()),
            None => bail!("Failed to get a current token"),
        }
    }

    fn previous(&mut self) -> Result<&Token> {
        match self.tokens.get(self.current - 1) {
            Some(t) => Ok(t),
            None => bail!("Failed to get a previous token"),
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
        self.advance()?;

        while !self.is_at_end() {
            if self.previous()?.token_type == TokenType::Semicolon {
                return Ok(());
            }
            if t.contains(&self.peek()?.token_type) {
                return Ok(());
            }
            self.advance()?;
        }

        Ok(())
    }
}
