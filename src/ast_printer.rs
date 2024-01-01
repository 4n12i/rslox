use crate::expr::Expr;
use crate::expr::Expr::*;
use anyhow::Result;

#[allow(dead_code)]
fn print_ast(e: Expr) -> Result<()> {
    match e {
        Binary(left, token, right) => {
            print!("(");
            print_ast(*left)?;
            print!("{}", token.lexeme);
            print_ast(*right)?;
            print!(")");
        }
        Grouping(expr) => {
            print!("(grouping");
            print_ast(*expr)?;
            print!(")");
        }
        Literal(token) => {
            print!("{}", token.lexeme);
        }
        Unary(token, expr) => {
            print!("(");
            print!("{}", token.lexeme);
            print_ast(*expr)?;
            print!(")");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::token::Literal;
    use crate::token::Token;
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print() {
        let e = Expr::Binary(
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
        );

        assert!(print_ast(e).is_ok());
    }
}
