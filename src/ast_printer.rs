use crate::expr::Expr;
use crate::expr::Expr::*;
use anyhow::{bail, Result};

pub fn format_ast(e: Expr) -> Result<String> {
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
        Literal(value) => value.to_string(),
        Unary(operator, right) => {
            format!("({} {})", operator.lexeme, format_ast(*right)?)
        }
        // TODO: Remove
        None => bail!("Expr is None"),
    };

    Ok(s)
}

#[cfg(test)]
mod tests {
    use crate::literal::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print_ast() {
        let e = format_ast(Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-", Literal::None, 1),
                Box::new(Expr::Literal(Literal::Num(123f64))),
            )),
            Token::new(TokenType::Star, "*", Literal::None, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Literal::Num(
                45.67f64,
            ))))),
        ));
        assert!(e.is_ok());
        println!("{}", e.unwrap());
    }
}
