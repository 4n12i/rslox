use crate::environment::Environment;
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

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new_global(),
        }
    }

    // pub fn run(expr: &Expr) -> Result<()> {
    //     match evaluate(expr) {
    //         Ok(value) => {
    //             println!("{value}");
    //             Ok(())
    //         }
    //         Err(error) => bail!("{error}"),
    //     }
    // }

    pub fn run(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }
}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<LoxValue> {
        match expr {
            Expr::Assign(name, value) => {
                let value = self.evaluate(value)?;
                self.environment.assign(name, &value)?;
                Ok(value)
            }
            Expr::Binary(left, operator, right) => {
                let left = self.evaluate(left)?;
                let right = self.evaluate(right)?;
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
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 - n2))
                        }
                        _ => bail!("Operands must be numbers"),
                    },
                    TokenType::Plus => match (left, right) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 + n2))
                        }
                        (LoxValue::String(s1), LoxValue::String(s2)) => {
                            Ok(LoxValue::String(s1 + &s2))
                        }
                        _ => bail!(RuntimeError::InvalidOperands.report(operator)),
                    },
                    TokenType::Slash => match (left, right) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 / n2))
                        }
                        _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                    },
                    TokenType::Star => match (left, right) {
                        (LoxValue::Number(n1), LoxValue::Number(n2)) => {
                            Ok(LoxValue::Number(n1 * n2))
                        }
                        _ => bail!(RuntimeError::NonNumericOperands.report(operator)),
                    },
                    _ => bail!("Error"), // Unreachable
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Unary(operator, right) => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(LoxValue::Boolean(!is_truthy(right))),
                    TokenType::Minus => match right {
                        LoxValue::Number(n) => Ok(LoxValue::Number(-n)),
                        _ => bail!(RuntimeError::NonNumericOperand.report(operator)),
                    },
                    _ => bail!("Error"), // Unreachable
                }
            }
            Expr::Variable(token) => self.environment.get(token),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                let previous = self.environment.clone();
                self.environment = Environment::new_local(&self.environment);

                for stmt in stmts {
                    if self.execute(stmt).is_err() {
                        self.environment = previous.clone();
                    }
                }

                self.environment = previous.clone();
                return Ok(());
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(expr)?);
            }
            Stmt::Var(token, expr) => {
                let value = match expr {
                    Some(initializer) => self.evaluate(initializer)?,
                    None => LoxValue::Nil,
                };
                self.environment.define(&token.lexeme, &value)?;
            }
        }
        Ok(())
    }
}

// Only false and nil are falsey, everything else is truthy
fn is_truthy(object: LoxValue) -> bool {
    match object {
        LoxValue::Boolean(b) => b,
        LoxValue::Nil => false,
        _ => true,
    }
}
