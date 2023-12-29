use crate::token_type::TokenType;
use anyhow::Result;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    literal: String,
    line: usize,
}

#[allow(dead_code)]
impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, literal: &str, line: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.to_string(),
            literal: literal.to_string(),
            line,
        }
    }

    fn get_string(&mut self) -> Result<String> {
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
    fn make_token() {
        let mut token = Token::new(TokenType::String, "test", "test", 1);
        assert_eq!(format!("String test test"), token.get_string().unwrap());
    }
}
