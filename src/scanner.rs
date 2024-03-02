use crate::literal::Literal;
use crate::result::Error;
use crate::result::Result;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::token_type::KEYWORDS;
use tracing::debug;

#[derive(Debug)]
pub struct Scanner {
    source: String,
    chars: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
}

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

    pub fn run(&mut self) -> Result<Vec<Token>> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            // At the beginning of the next lexeme.
            self.start = self.current;
            match self.scan_token() {
                Ok(Some(t)) => {
                    debug!("{t}");
                    tokens.push(t);
                }
                Err(e) => return Err(e),
                _ => (),
            }
        }
        tokens.push(Token::new(TokenType::Eof, "", Literal::Nil, self.line));

        Ok(tokens)
    }

    fn scan_token(&mut self) -> Result<Option<Token>> {
        let c = self.advance()?;

        let token = match c {
            '(' => self.create_token(TokenType::LeftParen)?,
            ')' => self.create_token(TokenType::RightParen)?,
            '{' => self.create_token(TokenType::LeftBrace)?,
            '}' => self.create_token(TokenType::RightBrace)?,
            ',' => self.create_token(TokenType::Comma)?,
            '.' => self.create_token(TokenType::Dot)?,
            '-' => self.create_token(TokenType::Minus)?,
            '+' => self.create_token(TokenType::Plus)?,
            ';' => self.create_token(TokenType::Semicolon)?,
            '*' => self.create_token(TokenType::Star)?,
            // Two characters token
            '!' => match self.is_match('=') {
                true => self.create_token(TokenType::BangEqual)?,
                false => self.create_token(TokenType::Bang)?,
            },
            '=' => match self.is_match('=') {
                true => self.create_token(TokenType::EqualEqual)?,
                false => self.create_token(TokenType::Equal)?,
            },
            '<' => match self.is_match('=') {
                true => self.create_token(TokenType::LessEqual)?,
                false => self.create_token(TokenType::Less)?,
            },
            '>' => match self.is_match('=') {
                true => self.create_token(TokenType::GreaterEqual)?,
                false => self.create_token(TokenType::Greater)?,
            },
            '/' => match self.is_match('/') {
                // A comment goes until the end of the line.
                true => {
                    while self.peek()? != '\n' && !self.is_at_end() {
                        self.advance()?;
                    }
                    return Ok(None);
                }
                false => self.create_token(TokenType::Slash)?,
            },
            ' ' | '\r' | '\t' => return Ok(None), // Ignore whitespace.
            '\n' => {
                self.line += 1;
                return Ok(None);
            }
            '"' => match self.get_string()? {
                Some(s) => self.create_token_with_literal(TokenType::String, Literal::String(s))?,
                None => return Ok(None),
            },
            _ => {
                if is_digit(c) {
                    let n = self.get_number()?;
                    self.create_token_with_literal(TokenType::Number, Literal::Number(n))?
                } else if is_alpha(c) {
                    let t = self.get_identifier()?;
                    self.create_token(t)?
                } else {
                    return Err(Error::Lexical(
                        self.line,
                        "Unexpected character.".to_string(),
                    ));
                }
            }
        };

        Ok(Some(token))
    }

    fn get_identifier(&mut self) -> Result<TokenType> {
        while is_alpha_numeric(self.peek()?) {
            self.advance()?;
        }

        let s: String = self
            .chars
            .get((self.start)..(self.current))
            .expect("Failed to get a string from source.")
            .iter()
            .collect();
        match KEYWORDS.get(&s as &str) {
            Some(t) => Ok(t.clone()),
            None => Ok(TokenType::Identifier),
        }
    }

    fn get_number(&mut self) -> Result<f64> {
        while is_digit(self.peek()?) {
            self.advance()?;
        }

        if self.peek()? == '.' && is_digit(self.peek_next()?) {
            self.advance()?;
            while is_digit(self.peek()?) {
                self.advance()?;
            }
        }

        let value = self.source[self.start..self.current]
            .parse::<f64>()
            .unwrap();
        Ok(value)
    }

    fn get_string(&mut self) -> Result<Option<String>> {
        while self.peek()? != '"' && !self.is_at_end() {
            if self.peek()? == '\n' {
                self.line += 1;
            }
            self.advance()?;
        }

        if self.is_at_end() {
            return Err(Error::Lexical(
                self.line,
                "Unterminated string.".to_string(),
            ));
        }

        // The closing `"`.
        self.advance()?;

        // Trim the surrounding quotes.
        let value = self
            .chars
            .get((self.start + 1)..(self.current - 1))
            .expect("Failed to get a string from source.")
            .iter()
            .collect();
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
        self.current >= self.chars.len()
    }

    fn advance(&mut self) -> Result<char> {
        let c = self.chars[self.current];
        self.current += 1;
        Ok(c)
    }

    fn peek(&mut self) -> Result<char> {
        if self.is_at_end() {
            return Ok('\0');
        }
        Ok(self.chars[self.current])
    }

    fn peek_next(&mut self) -> Result<char> {
        if self.current + 1 >= self.chars.len() {
            return Ok('\0');
        }
        Ok(self.chars[self.current + 1])
    }

    fn create_token(&mut self, token_type: TokenType) -> Result<Token> {
        self.create_token_with_literal(token_type, Literal::Nil)
    }

    fn create_token_with_literal(
        &mut self,
        token_type: TokenType,
        literal: Literal,
    ) -> Result<Token> {
        let lexeme: String = self
            .chars
            .get((self.start)..(self.current))
            .ok_or(Error::Lexical(
                self.line,
                "Failed to get a lexeme.".to_string(),
            ))?
            .iter()
            .collect();
        Ok(Token::new(token_type, &lexeme, literal, self.line))
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

    #[test]
    fn scan_tokens() {
        let src_addition = "1 + 2";
        let src_if_and_comment = "if (n1 + n2) <= 3 { // comment\n }";
        
        assert_eq!(
            Scanner::new(src_addition).run().unwrap(),
            vec![
                Token::new(TokenType::Number, "1", Literal::Number(1f64), 1),
                Token::new(TokenType::Plus, "+", Literal::Nil, 1),
                Token::new(TokenType::Number, "2", Literal::Number(2f64), 1),
                Token::new(TokenType::Eof, "", Literal::Nil, 1)
            ]
        );
        assert_eq!(
            Scanner::new(src_if_and_comment).run().unwrap(),
            vec![
                Token::new(TokenType::If, "if", Literal::Nil, 1),
                Token::new(TokenType::LeftParen, "(", Literal::Nil, 1),
                Token::new(TokenType::Identifier, "n1", Literal::Nil, 1),
                Token::new(TokenType::Plus, "+", Literal::Nil, 1),
                Token::new(TokenType::Identifier, "n2", Literal::Nil, 1),
                Token::new(TokenType::RightParen, ")", Literal::Nil, 1),
                Token::new(TokenType::LessEqual, "<=", Literal::Nil, 1),
                Token::new(TokenType::Number, "3", Literal::Number(3f64), 1),
                Token::new(TokenType::LeftBrace, "{", Literal::Nil, 1),
                Token::new(TokenType::RightBrace, "}", Literal::Nil, 2),
                Token::new(TokenType::Eof, "", Literal::Nil, 2)
            ]
        );
    }

    #[test]
    fn scan_token() {
        let src_plus = "+";
        let src_bang_equal = "!=";
        let src_whitespace = " ";
        let src_comment = "// comment\n";
        let src_slash= "/";
        let src_string= "\"string\"";
        let src_string_with_newline = "\"string\nstring\"";
        let src_decimal = "2.024";
        let src_or = "or";
        let src_identifier = "tmp";

        assert_eq!(
            Scanner::new(src_plus).scan_token().unwrap(),
            Some(Token::new(TokenType::Plus, src_plus, Literal::Nil, 1))
        );
        assert_eq!(
            Scanner::new(src_bang_equal).scan_token().unwrap(),
            Some(Token::new(
                TokenType::BangEqual,
                src_bang_equal,
                Literal::Nil,
                1
            ))
        );
        assert_eq!(Scanner::new(src_whitespace).scan_token().unwrap(), None);
        assert_eq!(Scanner::new(src_comment).scan_token().unwrap(), None);
        assert_eq!(
            Scanner::new(src_slash).scan_token().unwrap(),
            Some(Token::new(TokenType::Slash, src_slash, Literal::Nil, 1))
        );
        assert_eq!(
            Scanner::new(src_string).scan_token().unwrap(),
            Some(Token::new(
                TokenType::String,
                src_string,
                Literal::String("string".to_string()),
                1
            ))
        );
        assert_eq!(
            Scanner::new(src_string_with_newline).scan_token().unwrap(),
            Some(Token::new(
                TokenType::String,
                src_string_with_newline,
                Literal::String("string\nstring".to_string()),
                2
            ))
        );
        assert_eq!(
            Scanner::new(src_decimal).scan_token().unwrap(),
            Some(Token::new(
                TokenType::Number,
                src_decimal,
                Literal::Number(src_decimal.parse::<f64>().unwrap()),
                1
            ))
        );
        assert_eq!(
            Scanner::new(src_or).scan_token().unwrap(),
            Some(Token::new(TokenType::Or, src_or, Literal::Nil, 1))
        );
        assert_eq!(
            Scanner::new(src_identifier).scan_token().unwrap(),
            Some(Token::new(
                TokenType::Identifier,
                src_identifier,
                Literal::Nil,
                1
            ))
        );
    }
}
