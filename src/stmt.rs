use crate::expr::Expr;
use crate::token::Token;
use core::fmt;

#[derive(Debug)]
pub enum Stmt {
    Expression(Box<Expr>),
    Print(Box<Expr>),
    Var(Token, Option<Box<Expr>>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ast(self))
    }
}

fn format_ast(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Expression(expr) => format!("(; {})", expr),
        Stmt::Print(value) => format!("(print {})", value),
        Stmt::Var(name, initializer) => match initializer {
            Some(i) => format!("(var {} = {})", name.lexeme, i),
            None => format!("(var {})", name.lexeme),
        },
    }
}
