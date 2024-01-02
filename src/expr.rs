use crate::literal::Literal;
use crate::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>), // +, -, *, /, ==, !=, <, <=, >, >=
    Grouping(Box<Expr>),                 // ( and )
    Literal(Literal),                    // number, string, boolean, nil
    Unary(Token, Box<Expr>),             // !, -
}
