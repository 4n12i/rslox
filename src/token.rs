use crate::literal::Literal;
use crate::token_type::TokenType;
use anyhow::Result;

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
