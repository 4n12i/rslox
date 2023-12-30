use core::fmt;

use crate::token_type::TokenType;
use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: Literal,
    line: usize,
}

#[derive(Debug, PartialEq)]
pub enum Literal {
    Str(String),
    Num(f64),
    None,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Str(s) => write!(f, "{s}"),
            Self::Num(n) => write!(f, "{n}"),
            Self::None => write!(f, "null"),
        }
    }
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal,
            line,
        }
    }

    pub fn get_string(&mut self) -> Result<String> {
        Ok(format!(
            "{:?} {} {}",
            self.token_type, self.lexeme, self.literal
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_token() {
        let mut token = Token::new(
            TokenType::String,
            "test",
            Literal::Str("test".to_string()),
            1,
        );
        assert_eq!(format!("String test test"), token.get_string().unwrap());
    }
}
