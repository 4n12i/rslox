use crate::literal::Literal;
use crate::token::Token;
use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>), // +, -, *, /, ==, !=, <, <=, >, >=
    Grouping(Box<Expr>),                 // ( and )
    Literal(Literal),                    // number, string, boolean, nil
    Unary(Token, Box<Expr>),             // !, -
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ast(self))
    }
}

fn format_ast(expr: &Expr) -> String {
    match expr {
        Expr::Binary(left, operator, right) => {
            format!(
                "({} {} {})",
                operator.lexeme,
                format_ast(left),
                format_ast(right)
            )
        }
        Expr::Grouping(expr) => {
            format!("(group {})", format_ast(expr))
        }
        Expr::Literal(value) => value.to_string(),
        Expr::Unary(operator, right) => {
            format!("({} {})", operator.lexeme, format_ast(right))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print_ast() {
        let e = format_ast(&Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-", Literal::Nil, 1),
                Box::new(Expr::Literal(Literal::Number(123f64))),
            )),
            Token::new(TokenType::Star, "*", Literal::Nil, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Number(
                45.67f64,
            ))))),
        ));
        println!("{e}");
    }
}
