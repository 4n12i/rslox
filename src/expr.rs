use crate::literal::Literal;
use crate::token::Token;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>), // +, -, *, /, ==, !=, <, <=, >, >=
    Grouping(Box<Expr>),                 // ( and )
    Literal(Literal),                    // number, string, boolean, nil
    Unary(Token, Box<Expr>),             // !, -
    None,
}
