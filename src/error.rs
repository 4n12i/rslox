use crate::token::Token;
use crate::token_type::TokenType;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("[line {line}] Error{place:?}: {message:?}")]
    _Syntax {
        line: usize,
        place: String,
        message: String,
    },

    #[error("[line {line}] Unexpected character")]
    Lexical { line: usize },

    #[error("[line {line}] Unterminated string")]
    StringEnd { line: usize },
}

pub fn get_parse_error(token: &Token, message: &str) -> String {
    let place = if token.token_type == TokenType::Eof {
        " at end".to_string()
    } else {
        format!(" at '{}'", token.lexeme)
    };

    format!("[line {}] Error{}: {}", token.line, place, message)
}
