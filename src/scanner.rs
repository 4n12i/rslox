use crate::lox::ErrorType;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;
use tracing::error;

#[allow(dead_code)]
#[derive(Debug)]
struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

#[allow(dead_code)]
impl Scanner {
    fn new(source: String) -> Self {
        Scanner {
            source,
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn scan_tokens(&mut self) -> Result<()> {
        while self.is_at_end() {
            self.start = self.current;
            self.scan_tokens()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, "", "null", self.line));

        Ok(())
    }

    fn scan_token(&mut self) -> Result<()> {
        let c = self.advance_one_char()?;

        match c {
            '(' => self.add_token_with_one_symbol(TokenType::LeftParen),
            ')' => self.add_token_with_one_symbol(TokenType::RightParen),
            '{' => self.add_token_with_one_symbol(TokenType::LeftBrace),
            '}' => self.add_token_with_one_symbol(TokenType::RightBrace),
            ',' => self.add_token_with_one_symbol(TokenType::Comma),
            '.' => self.add_token_with_one_symbol(TokenType::Dot),
            '-' => self.add_token_with_one_symbol(TokenType::Minus),
            '+' => self.add_token_with_one_symbol(TokenType::Plus),
            ';' => self.add_token_with_one_symbol(TokenType::Semicolon),
            '*' => self.add_token_with_one_symbol(TokenType::Star),
            _ => {
                error!("{}", ErrorType::Lexical { line: self.line });
                Ok(())
            }
        }
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance_one_char(&mut self) -> Result<char> {
        self.current += 1;
        match self.source.chars().nth(self.current) {
            Some(c) => Ok(c),
            None => bail!("Failed to get a next character"),
        }
    }

    fn add_token_with_one_symbol(&mut self, token_type: TokenType) -> Result<()> {
        self.add_token(token_type, "null")
    }

    fn add_token(&mut self, token_type: TokenType, literal: &str) -> Result<()> {
        match self.source.get(self.start..self.current) {
            Some(t) => self
                .tokens
                .push(Token::new(token_type, t, literal, self.line)),
            None => bail!("Failed to get source code"),
        }

        Ok(())
    }
}
