use std::collections::HashMap;

use crate::error::ErrorType;
use crate::literal::Literal;
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

        while !self.is_at_end() {
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
            '(' => self.add_token(TokenType::LeftParen)?,
            ')' => self.add_token(TokenType::RightParen)?,
            '{' => self.add_token(TokenType::LeftBrace)?,
            '}' => self.add_token(TokenType::RightBrace)?,
            ',' => self.add_token(TokenType::Comma)?,
            '.' => self.add_token(TokenType::Dot)?,
            '-' => self.add_token(TokenType::Minus)?,
            '+' => self.add_token(TokenType::Plus)?,
            ';' => self.add_token(TokenType::Semicolon)?,
            '*' => self.add_token(TokenType::Star)?,
            // Two characters token
            '!' => match self.is_match('=') {
                true => self.add_token(TokenType::BangEqual)?,
                false => self.add_token(TokenType::Bang)?,
            },
            '=' => match self.is_match('=') {
                true => self.add_token(TokenType::EqualEqual)?,
                false => self.add_token(TokenType::Equal)?,
            },
            '<' => match self.is_match('=') {
                true => self.add_token(TokenType::LessEqual)?,
                false => self.add_token(TokenType::Less)?,
            },
            '>' => match self.is_match('=') {
                true => self.add_token(TokenType::GreaterEqual)?,
                false => self.add_token(TokenType::Greater)?,
            },
            '/' => match self.is_match('/') {
                // A comment goes until the end of the line.
                true => {
                    while self.peek_one_ahead()? != '\n' && !self.is_at_end() {
                        self.advance_one_char()?;
                    }
                    return Ok(None);
                }
                false => self.add_token(TokenType::Slash)?,
            },
            ' ' | '\r' | '\t' => return Ok(None), // Ignore whitespace.
            '\n' => {
                self.line += 1;
                return Ok(None);
            }
            '"' => match self.get_string_literal()? {
                Some(s) => self.add_token_with_literal(TokenType::String, Literal::Str(s))?,
                None => return Ok(None),
            },
            _ => {
                if is_digit(c) {
                    let n = self.get_number_literal()?;
                    self.add_token_with_literal(TokenType::Number, Literal::Num(n))?
                } else if is_alpha(c) {
                    let t = self.get_identifier()?;
                    self.add_token(t)?
                } else {
                    error!("{}", ErrorType::Lexical { line: self.line });
                    return Ok(None);
                }
            }
        };

        Ok(Some(token))
    }

    fn get_identifier(&mut self) -> Result<TokenType> {
        while is_alpha_numeric(self.peek_one_ahead()?) {
            self.advance_one_char()?;
        }

        let text = self.source[self.start..self.current].to_string();
        match KEYWORDS.get(&text as &str) {
            Some(t) => Ok(t.clone()),
            None => Ok(TokenType::Identifier),
        }
    }

    fn get_number_literal(&mut self) -> Result<f64> {
        while is_digit(self.peek_one_ahead()?) {
            self.advance_one_char()?;
        }

        if self.peek_one_ahead()? == '.' && is_digit(self.peek_two_ahead()?) {
            self.advance_one_char()?;
            while is_digit(self.peek_one_ahead()?) {
                self.advance_one_char()?;
            }
        }

        let value = self.source[self.start..self.current].parse::<f64>()?;
        Ok(value)
    }

    fn get_string_literal(&mut self) -> Result<Option<String>> {
        while self.peek_one_ahead()? != '"' && !self.is_at_end() {
            if self.peek_one_ahead()? == '\n' {
                self.line += 1;
            }
            self.advance_one_char()?;
        }

        if self.is_at_end() {
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
        if self.is_at_end() {
            return false;
        }
        if self.chars[self.current] != expected {
            return false;
        }
        self.current += 1;
        true
    }

    fn is_at_end(&mut self) -> bool {
        self.current >= self.source.len()
    }

    fn advance_one_char(&mut self) -> Result<char> {
        let c = self.chars[self.current];
        self.current += 1;
        Ok(c)
    }

    fn peek_one_ahead(&mut self) -> Result<char> {
        if self.is_at_end() {
            return Ok('\0');
        }
        Ok(self.chars[self.current])
    }

    fn peek_two_ahead(&mut self) -> Result<char> {
        if self.is_at_end() {
            return Ok('\0');
        }
        Ok(self.chars[self.current + 1])
    }

    fn add_token(&mut self, token_type: TokenType) -> Result<Token> {
        self.add_token_with_literal(token_type, Literal::None)
    }

    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Literal) -> Result<Token> {
        match self.source.get(self.start..self.current) {
            Some(t) => Ok(Token::new(token_type, t, literal, self.line)),
            None => bail!("Failed to get source code"),
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}
fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[cfg(test)]
mod tests {
    use super::*;

    const SRC_PLUS: &str = "+";
    const SRC_BANG_EQUAL: &str = "!=";
    const SRC_WHITESPACE: &str = " ";
    const SRC_COMMENT: &str = "// comment\n";
    const SRC_STASH: &str = "/";
    const SRC_STRING: &str = "\"string\"";
    const SRC_STRING_WITH_NEWLINE: &str = "\"string\nstring\"";
    const SRC_DECIMAL: &str = "2.024";
    const SRC_OR: &str = "or";
    const SRC_IDENTIFIER: &str = "tmp";

    const SRC_ADDITION: &str = "1 + 2";
    const SRC_IF_AND_COMMENT: &str = "if (n1 + n2) <= 3 { // comment\n }";

    #[test]
    fn scan_tokens() {
        assert_eq!(
            Scanner::new(SRC_ADDITION).scan_tokens().unwrap(),
            vec![
                Token::new(TokenType::Number, "1", Literal::Num(1f64), 1),
                Token::new(TokenType::Plus, "+", Literal::None, 1),
                Token::new(TokenType::Number, "2", Literal::Num(2f64), 1),
                Token::new(TokenType::Eof, "", Literal::None, 1)
            ]
        );
        assert_eq!(
            Scanner::new(SRC_IF_AND_COMMENT).scan_tokens().unwrap(),
            vec![
                Token::new(TokenType::If, "if", Literal::None, 1),
                Token::new(TokenType::LeftParen, "(", Literal::None, 1),
                Token::new(TokenType::Identifier, "n1", Literal::None, 1),
                Token::new(TokenType::Plus, "+", Literal::None, 1),
                Token::new(TokenType::Identifier, "n2", Literal::None, 1),
                Token::new(TokenType::RightParen, ")", Literal::None, 1),
                Token::new(TokenType::LessEqual, "<=", Literal::None, 1),
                Token::new(TokenType::Number, "3", Literal::Num(3f64), 1),
                Token::new(TokenType::LeftBrace, "{", Literal::None, 1),
                Token::new(TokenType::RightBrace, "}", Literal::None, 2),
                Token::new(TokenType::Eof, "", Literal::None, 2)
            ]
        );
    }

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
                Literal::Str("string".to_string()),
                1
            ))
        );
        assert_eq!(
            Scanner::new(SRC_STRING_WITH_NEWLINE).scan_token().unwrap(),
            Some(Token::new(
                TokenType::String,
                SRC_STRING_WITH_NEWLINE,
                Literal::Str("string\nstring".to_string()),
                2
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
}
