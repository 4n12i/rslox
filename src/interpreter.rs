use crate::expr::Expr;
use crate::literal::Literal as LoxValue;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;

pub struct Interpreter {}

impl Interpreter {
    pub fn run(e: &Expr) -> Result<()> {
        evaluate(e)?;
        Ok(())
    }
}

fn evaluate(e: &Expr) -> Result<LoxValue> {
    match e {
        Expr::Binary(left, operator, right) => {
            let left = evaluate(left)?;
            let right = evaluate(right)?;
            match operator.token_type {
                TokenType::BangEqual => Ok(LoxValue::Boolean(left.ne(&right))),
                TokenType::EqualEqual => Ok(LoxValue::Boolean(left.eq(&right))),
                TokenType::Greater => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.gt(&n2)))
                    }
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::GreaterEqual => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.ge(&n2)))
                    }
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::Less => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.lt(&n2)))
                    }
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::LessEqual => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.le(&n2)))
                    }
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::Minus => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 - n2)),
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::Plus => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 - n2)),
                    (LoxValue::String(s1), LoxValue::String(s2)) => Ok(LoxValue::String(s1 + &s2)),
                    _ => bail!("Operands must be two numbers or two strings"),
                },
                TokenType::Slash => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 / n2)),
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::Star => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 * n2)),
                    _ => bail!("Operands must be numbers"),
                },
                _ => bail!("Error"),
            }
        }
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Literal(_value) => {
            bail!("Unimplemented") // Ok(value)
        }
        Expr::Unary(operator, right) => {
            let right = evaluate(right)?;
            match operator.token_type {
                TokenType::Bang => Ok(LoxValue::Boolean(!is_truthy(right))),
                TokenType::Minus => match right {
                    LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                    _ => bail!("Operand must be a number"),
                },
                _ => bail!("Error"),
            }
        }
    }
}

/// Only false and nil are falsey, everything else is truthy
fn is_truthy(object: LoxValue) -> bool {
    match object {
        LoxValue::Boolean(b) => b,
        LoxValue::Nil => false,
        _ => true,
    }
}
