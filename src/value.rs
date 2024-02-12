use crate::function::LoxFunction;
use crate::function::NativeFunction;
use crate::literal::Literal;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Value {
    Boolean(bool),
    Number(f64),
    String(String),
    Nil,
    LoxFunction(LoxFunction),
    NativeFunction(NativeFunction),
}

impl From<Literal> for Value {
    fn from(value: Literal) -> Self {
        match value {
            Literal::Number(n) => Value::Number(n),
            Literal::String(s) => Value::String(s),
            Literal::Nil => Value::Nil,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Boolean(b) => write!(f, "{b}"),
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "nil"),
            Self::LoxFunction(fun) => write!(f, "{fun}"),
            Self::NativeFunction(fun) => write!(f, "{fun}"),
        }
    }
}

impl Value {
    // Only false and nil are falsey, everything else is truthy
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Boolean(b) => *b,
            Self::Nil => false,
            _ => true,
        }
    }
}
