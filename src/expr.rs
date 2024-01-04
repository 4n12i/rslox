use crate::literal::Literal;
use crate::token::Token;
use anyhow::Result;
use core::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>), // +, -, *, /, ==, !=, <, <=, >, >=
    Grouping(Box<Expr>),                 // ( and )
    Literal(Literal),                    // number, string, boolean, nil
    Unary(Token, Box<Expr>),             // !, -
}

impl Expr {
    fn format_ast(e: &Expr) -> Result<String> {
        let s = match e {
            Expr::Binary(left, operator, right) => {
                format!(
                    "({} {} {})",
                    operator.lexeme,
                    Self::format_ast(left)?,
                    Self::format_ast(right)?
                )
            }
            Expr::Grouping(expr) => {
                format!("(group {})", Self::format_ast(expr)?)
            }
            Expr::Literal(value) => value.to_string(),
            Expr::Unary(operator, right) => {
                format!("({} {})", operator.lexeme, Self::format_ast(right)?)
            }
        };

        Ok(s)
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", Self::format_ast(self).unwrap())
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
        assert!(e.is_ok());
        println!("{}", e.unwrap());
    }
}
