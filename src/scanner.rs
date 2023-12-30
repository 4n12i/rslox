use std::collections::HashMap;

use crate::lox::ErrorType;
use crate::token::Literal;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;
use once_cell::sync::Lazy;
use tracing::error;

static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("for", TokenType::For),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ])
});

#[derive(Debug)]
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

#[allow(dead_code)]
impl Scanner {
    pub fn new(source: &str) -> Self {
        Scanner {
            source: source.to_string(),
            chars: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while is_at_end(self.current, &self.source) {
            // At the beginning of the next lexeme.
            self.start = self.current;
            if let Some(t) = self.scan_token()? {
                tokens.push(t);
            }
        }

        tokens.push(Token::new(TokenType::Eof, "", Literal::None, self.line));

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let c = self.advance_one_char()?;

        let token = match c {
            '(' => self.add_token_with_one_symbol(TokenType::LeftParen)?,
            ')' => self.add_token_with_one_symbol(TokenType::RightParen)?,
            '{' => self.add_token_with_one_symbol(TokenType::LeftBrace)?,
            '}' => self.add_token_with_one_symbol(TokenType::RightBrace)?,
            ',' => self.add_token_with_one_symbol(TokenType::Comma)?,
            '.' => self.add_token_with_one_symbol(TokenType::Dot)?,
            '-' => self.add_token_with_one_symbol(TokenType::Minus)?,
            '+' => self.add_token_with_one_symbol(TokenType::Plus)?,
            ';' => self.add_token_with_one_symbol(TokenType::Semicolon)?,
            '*' => self.add_token_with_one_symbol(TokenType::Star)?,
            // Two characters token
            '!' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::BangEqual)?,
                false => self.add_token_with_one_symbol(TokenType::Bang)?,
            },
            '=' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::EqualEqual)?,
                false => self.add_token_with_one_symbol(TokenType::Equal)?,
            },
            '<' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::LessEqual)?,
                false => self.add_token_with_one_symbol(TokenType::Less)?,
            },
            '>' => match self.is_match('=') {
                true => self.add_token_with_one_symbol(TokenType::GreaterEqual)?,
                false => self.add_token_with_one_symbol(TokenType::Greater)?,
            },
            // Slash
            '/' => match self.is_match('/') {
                // A comment goes until the end of the line.
                true => {
                    while self.peek_one_ahead()? != '\n' && !is_at_end(self.current, &self.source) {
                        self.advance_one_char()?;
                    }
                    return Ok(None);
                }
                false => self.add_token_with_one_symbol(TokenType::Slash)?,
            },
            // Whitespace
            ' ' | '\r' | '\t' => return Ok(None), // Ignore whitespace.
            '\n' => {
                self.line += 1;
                return Ok(None);
            }
            // String
            '"' => match self.find_string_literal()? {
                Some(s) => self.add_token(TokenType::String, Literal::Str(s))?,
                None => return Ok(None),
            },
            _ => {
                if c.is_ascii_digit() {
                    let n = self.find_number_literal()?;
                    self.add_token(TokenType::Number, Literal::Num(n))?
                } else if is_alpha(c) {
                    let t = self.find_identifier()?;
                    self.add_token_with_one_symbol(t)?
                } else {
                    error!("{}", ErrorType::Lexical { line: self.line });
                    return Ok(None);
                }
            }
        };

        Ok(Some(token))
    }

    fn find_identifier(&mut self) -> Result<TokenType> {
        while is_alpha_numeric(self.peek_one_ahead()?) {
            self.advance_one_char()?;
        }

        let text = self.source[self.start..self.current].to_string();
        match KEYWORDS.get(&text as &str) {
            Some(t) => Ok(t.clone()),
            None => Ok(TokenType::Identifier),
        }
    }

    fn find_number_literal(&mut self) -> Result<f64> {
        while self.peek_one_ahead()?.is_ascii_digit() {
            self.advance_one_char()?;
        }

        if self.peek_one_ahead()? == '.' && self.peek_two_ahead()?.is_ascii_digit() {
            self.advance_one_char()?;
            while self.peek_one_ahead()?.is_ascii_digit() {
                self.advance_one_char()?;
            }
        }

        let value = self.source[self.start..self.current].parse::<f64>()?;
        Ok(value)
    }

    fn find_string_literal(&mut self) -> Result<Option<String>> {
        while self.peek_one_ahead()? != '"' && !is_at_end(self.current, &self.source) {
            if self.peek_one_ahead()? == '\n' {
                self.line += 1;
            }
            self.advance_one_char()?;
        }

        if is_at_end(self.current, &self.source) {
            error!("{}", ErrorType::StringEnd { line: self.line });
            return Ok(None);
        }

        // The closing `"`.
        self.advance_one_char()?;

        // Trim the surrounding quotes.
        let value = self.source[self.start..self.current]
            .trim_matches('"')
            .to_string();
        Ok(Some(value))
    }

    fn is_match(&mut self, expected: char) -> bool {
        if is_at_end(self.current, &self.source) {
            return false;
        }
        let c = self.chars[self.current];
        if c != expected {
            return false;
        }
        self.current += 1;

        true
    }

    fn advance_one_char(&mut self) -> Result<char> {
        let c = self.chars[self.current];
        self.current += 1;

        Ok(c)
    }

    fn peek_one_ahead(&mut self) -> Result<char> {
        if is_at_end(self.current, &self.source) {
            return Ok('\0');
        }

        let c = self.chars[self.current];
        Ok(c)
    }

    fn peek_two_ahead(&mut self) -> Result<char> {
        if is_at_end(self.current + 1, &self.source) {
            return Ok('\0');
        }

        let c = self.chars[self.current + 1];
        Ok(c)
    }

    fn add_token_with_one_symbol(&mut self, token_type: TokenType) -> Result<Token> {
        self.add_token(token_type, Literal::None)
    }

    fn add_token(&mut self, token_type: TokenType, literal: Literal) -> Result<Token> {
        match self.source.get(self.start..self.current) {
            Some(t) => Ok(Token::new(token_type, t, literal, self.line)),
            None => bail!("Failed to get source code"),
        }
    }
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

fn is_at_end(current: usize, source: &str) -> bool {
    current >= source.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC_PLUS: &str = r"+";
    const SRC_BANG_EQUAL: &str = r"!=";
    const SRC_WHITESPACE: &str = r" ";
    const SRC_COMMENT: &str = r"// comment\n";
    const SRC_STASH: &str = r"/";
    const SRC_STRING: &str = r#""string""#;
    const SRC_STRING_WITH_NEWLINE: &str = r#""string\nstring""#;
    const SRC_DECIMAL: &str = r"2.024";
    const SRC_OR: &str = r"or";
    const SRC_IDENTIFIER: &str = r"tmp";

    #[test]
    fn scan_token() {
        assert_eq!(
            Scanner::new(SRC_PLUS).scan_token().unwrap(),
            Some(Token::new(TokenType::Plus, SRC_PLUS, Literal::None, 1))
        );
        assert_eq!(
            Scanner::new(SRC_BANG_EQUAL).scan_token().unwrap(),
            Some(Token::new(
                TokenType::BangEqual,
                SRC_BANG_EQUAL,
                Literal::None,
                1
            ))
        );
        assert_eq!(Scanner::new(SRC_WHITESPACE).scan_token().unwrap(), None);
        assert_eq!(Scanner::new(SRC_COMMENT).scan_token().unwrap(), None);
        assert_eq!(
            Scanner::new(SRC_STASH).scan_token().unwrap(),
            Some(Token::new(TokenType::Slash, SRC_STASH, Literal::None, 1))
        );
        assert_eq!(
            Scanner::new(SRC_STRING).scan_token().unwrap(),
            Some(Token::new(
                TokenType::String,
                SRC_STRING,
                Literal::Str(r"string".to_string()),
                1
            ))
        );
        assert_eq!(
            Scanner::new(SRC_STRING_WITH_NEWLINE).scan_token().unwrap(),
            Some(Token::new(
                TokenType::String,
                SRC_STRING_WITH_NEWLINE,
                Literal::Str(r"string\nstring".to_string()),
                1
            ))
        );

        assert_eq!(
            Scanner::new(SRC_DECIMAL).scan_token().unwrap(),
            Some(Token::new(
                TokenType::Number,
                SRC_DECIMAL,
                Literal::Num(SRC_DECIMAL.parse::<f64>().unwrap()),
                1
            ))
        );
        assert_eq!(
            Scanner::new(SRC_OR).scan_token().unwrap(),
            Some(Token::new(TokenType::Or, SRC_OR, Literal::None, 1))
        );
        assert_eq!(
            Scanner::new(SRC_IDENTIFIER).scan_token().unwrap(),
            Some(Token::new(
                TokenType::Identifier,
                SRC_IDENTIFIER,
                Literal::None,
                1
            ))
        );
    }

    #[test]
    fn check_is_at_end() {
        assert!(!is_at_end(1, "1 + 2"));
        assert!(is_at_end(5, "1 + 2"));
        assert!(is_at_end(10, "1 + 2"));
    }
}
