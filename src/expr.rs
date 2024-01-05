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

impl Expr {
    fn format_ast(&self) -> String {
        match self {
            Expr::Binary(left, operator, right) => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    left.format_ast(),
                    right.format_ast()
                )
            }
            Expr::Grouping(expr) => {
                format!("(group {})", expr.format_ast())
            }
            Expr::Literal(value) => value.to_string(),
            Expr::Unary(operator, right) => {
                format!("({} {})", operator.lexeme, right.format_ast())
            }
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format_ast())
    }
}

#[cfg(test)]
mod tests {
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print_ast() {
        let e = Expr::format_ast(&Expr::Binary(
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
