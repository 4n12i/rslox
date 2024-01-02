use core::fmt;

use crate::literal::Literal;
use crate::token_type::TokenType;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Literal,
    pub line: usize,
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
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.token_type, self.lexeme, self.literal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn generate_token() {
        let token = Token::new(
            TokenType::String,
            "test",
            Literal::Str("test".to_string()),
            1,
        );
        assert_eq!("String test test".to_string(), token.to_string());
    }
}
