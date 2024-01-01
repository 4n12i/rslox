use crate::expr::Expr;
use crate::expr::Expr::*;
use anyhow::Result;

#[allow(dead_code)]
fn format_ast(e: Expr) -> Result<String> {
    let s = match e {
        Binary(left, operator, right) => {
            format!(
                "({} {} {})",
                operator.lexeme,
                format_ast(*left)?,
                format_ast(*right)?
            )
        }
        Grouping(expr) => {
            format!("(group {})", format_ast(*expr)?)
        }
        Literal(value) => value.lexeme,
        Unary(operator, right) => {
            format!("({} {})", operator.lexeme, format_ast(*right)?)
        }
    };

    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::token::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print_ast() {
        let e = format_ast(Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-", Literal::None, 1),
                Box::new(Expr::Literal(Token::new(
                    TokenType::Number,
                    "123",
                    Literal::Num(123f64),
                    1,
                ))),
            )),
            Token::new(TokenType::Star, "*", Literal::None, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Token::new(
                TokenType::Number,
                "45.67",
                Literal::Num(45.67f64),
                1,
            ))))),
        ));
        assert!(e.is_ok());
        println!("{}", e.unwrap());
    }
}
