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
            '!' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::BangEqual),
                false => self.add_token_with_one_symbol(TokenType::Bang),
            },
            '=' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::EqualEqual),
                false => self.add_token_with_one_symbol(TokenType::Equal),
            },
            '<' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::LessEqual),
                false => self.add_token_with_one_symbol(TokenType::Less),
            },
            '>' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::GreaterEqual),
                false => self.add_token_with_one_symbol(TokenType::Greater),
            },
            '/' => {
                match self.is_match('/') {
                    // If a comment exists
                    true => {
                        while self.peek_one_char()? != '\n' && !self.is_at_end() {
                            self.advance_one_char()?;
                        }
                        Ok(())
                    }
                    false => self.add_token_with_one_symbol(TokenType::Slash),
                }
            }
            ' ' | '\r' | '\t' => Ok(()), // Ignore whitespace
            '\n' => {
                self.line += 1;
                Ok(())
            }
            _ => {
                error!("{}", ErrorType::Lexical { line: self.line });
                Ok(())
            }
        }
    }

    fn is_match(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        let c = match self.source.chars().nth(self.current) {
            Some(c) => c,
            // TODO
            None => return false,
        };
        if c != expected {
            return false;
        }
        self.current += 1;

        true
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

    fn peek_one_char(&mut self) -> Result<char> {
        if self.is_at_end() {
            return Ok('\0');
        }

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
