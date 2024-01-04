use anyhow::Result;

use crate::expr::Expr;
use crate::literal::Literal;
use crate::token_type::TokenType;

#[allow(dead_code)]
fn evaluate(e: &Expr) -> Result<()> {
    match e {
        Expr::Binary(left, operator, right) => {
            let _left = evaluate(left);
            let _right = evaluate(right);
            match operator.token_type {
                TokenType::BangEqual => Ok(()),    // Ok(!is_equal(left, right))
                TokenType::EqualEqual => Ok(()),   // Ok(is_equal(left, right))
                TokenType::Greater => Ok(()),      // Ok(left > right)
                TokenType::GreaterEqual => Ok(()), // Ok(left >= right)
                TokenType::Less => Ok(()),         // Ok(left < right)
                TokenType::LessEqual => Ok(()),    // Ok(left <= right)
                TokenType::Minus => Ok(()),        // Ok(left - right as f64)
                TokenType::Plus => {
                    // if type_of(left) == type_of(right) == "f64" { Ok(left + right as f64) }
                    // if type_of(left) == type_of(right) == "String" { Ok(left + right as String) }
                    Ok(())
                }
                TokenType::Slash => Ok(()), // Ok(left / right as f64)
                TokenType::Star => Ok(()),  // Ok(left * right as f64)
                _ => Ok(()),
            }
        }
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Literal(_value) => {
            Ok(()) // Ok(value)
        }
        Expr::Unary(operator, right) => {
            let _right = evaluate(right);
            match operator.token_type {
                TokenType::Bang => Ok(()),  // Ok(!is_truthy(right))
                TokenType::Minus => Ok(()), // Ok(-right)
                _ => Ok(()),
            }
        }
    }
}

fn _is_truthy(object: &Literal) -> bool {
    object != &Literal::None
    // if type_of(object) == "bool" { return object as bool; }
}

fn _is_equal(a: &Literal, b: &Literal) -> bool {
    a.eq(b)
}
