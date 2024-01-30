use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use std::fmt;
use std::io;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Usage,
    IO(io::Error),
    Lexical(usize, String), // Scanner
    Parse(Token, String),   // Parser
    Runtime(Token, String), // Interpreter
    Return(Value),
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::IO(value)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Usage => write!(f, "Usage: rslox [script]"),
            Self::IO(error) => error.fmt(f),
            Self::Lexical(line, message) => write!(f, "[line {}] Error: {}", line, message),
            Self::Parse(token, message) => {
                let place = match token.token_type {
                    TokenType::Eof => " at end".to_string(),
                    _ => format!(" at '{}'", token.lexeme),
                };
                write!(f, "[line {}] Error{}: {}", token.line, place, message)
            }
            Self::Runtime(token, message) => write!(f, "{}\n[line {}]", message, token.line),
            Self::Return(_) => write!(f, "Unexpected return statement."),
        }
    }
}
