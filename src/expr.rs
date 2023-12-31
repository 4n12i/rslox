use crate::token::Literal as LiteralType;
use crate::token::Token;

pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Literal(Box<Literal>),
    Unary(Box<Unary>),
}

pub trait Visitor {
    fn accept(&self) {
        println!("test");
    }
}

#[allow(dead_code)]
pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    fn _new(left: Expr, operator: Token, right: Expr) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

#[allow(dead_code)]
pub struct Grouping {
    expression: Expr,
}

impl Grouping {
    fn _new(expression: Expr) -> Self {
        Grouping { expression }
    }
}

#[allow(dead_code)]
pub struct Literal {
    value: LiteralType,
}

impl Literal {
    fn _new(value: LiteralType) -> Self {
        Literal { value }
    }
}

#[allow(dead_code)]
pub struct Unary {
    operator: Token,
    right: Expr,
}

impl Unary {
    fn _new(operator: Token, right: Expr) -> Self {
        Unary { operator, right }
    }
}
