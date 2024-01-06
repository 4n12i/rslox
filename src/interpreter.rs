use crate::expr::Expr;
use crate::literal::Literal as LoxValue;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use anyhow::bail;
use anyhow::Result;
use core::fmt;

#[derive(Debug)]
enum RuntimeError {
    InvalidOperands,
    NonNumericOperand,
    NonNumericOperands,
}

impl RuntimeError {
    fn report(&self, t: &Token) -> String {
        format!("{}\n[line {}]", self, t.line)
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidOperands => write!(f, "Operands must be two numbers or two strings"),
            Self::NonNumericOperand => write!(f, "Operand must be a number"),
            Self::NonNumericOperands => write!(f, "Operands must be numbers"),
        }
    }
}

pub struct Interpreter {}

impl Interpreter {
    pub fn _run(expr: &Expr) -> Result<()> {
        match evaluate(expr) {
            Ok(value) => {
                println!("{value}");
                Ok(())
            }
            Err(error) => bail!("{error}"),
        }
    }

    pub fn run(statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            execute(statement)?;
        }
        Ok(())
    }
}

fn evaluate(expr: &Expr) -> Result<LoxValue> {
    match expr {
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
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                TokenType::GreaterEqual => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.ge(&n2)))
                    }
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                TokenType::Less => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.lt(&n2)))
                    }
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                TokenType::LessEqual => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                        Ok(LoxValue::Boolean(n1.le(&n2)))
                    }
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                TokenType::Minus => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 - n2)),
                    _ => bail!("Operands must be numbers"),
                },
                TokenType::Plus => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 + n2)),
                    (LoxValue::String(s1), LoxValue::String(s2)) => Ok(LoxValue::String(s1 + &s2)),
                    _ => bail!(RuntimeError::InvalidOperands.report(operator)),
                },
                TokenType::Slash => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 / n2)),
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                TokenType::Star => match (left, right) {
                    (LoxValue::Number(n1), LoxValue::Number(n2)) => Ok(LoxValue::Number(n1 * n2)),
                    _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                },
                _ => bail!("Error"),
            }
        }
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Literal(value) => Ok(value.clone()),
        Expr::Unary(operator, right) => {
            let right = evaluate(right)?;
            match operator.token_type {
                TokenType::Bang => Ok(LoxValue::Boolean(!is_truthy(right))),
                TokenType::Minus => match right {
                    LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                    _ => bail!(RuntimeError::NonNumericOperand.report(operator)),
                },
                _ => bail!("Error"),
            }
        }
    }
}

fn execute(stmt: &Stmt) -> Result<()> {
    match stmt {
        Stmt::Expression(e) => {
            evaluate(e)?;
        }
        Stmt::Print(e) => {
            println!("{}", evaluate(e)?);
        }
    }
    Ok(())
}

/// Only false and nil are falsey, everything else is truthy
fn is_truthy(object: LoxValue) -> bool {
    match object {
        LoxValue::Boolean(b) => b,
        LoxValue::Nil => false,
        _ => true,
    }
}
