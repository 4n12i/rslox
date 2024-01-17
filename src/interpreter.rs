use crate::callable::Callable;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::function::Function;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::token_type::TokenType;
use crate::value::Value;
use anyhow::bail;
use anyhow::Result;
use tracing::debug;

pub struct Interpreter {
    pub globals: Environment,
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        // globals.define("clock", )
        Self {
            globals: Environment::new_global(),
            environment: Environment::new_global(),
        }
    }

    pub fn run(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            debug!("[interpreter_run] stmt >> {}", statement);
            self.execute(statement)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
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
                    TokenType::BangEqual => Ok(Value::Boolean(left.ne(&right))),
                    TokenType::EqualEqual => Ok(Value::Boolean(left.eq(&right))),
                    TokenType::Greater => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.gt(&n2))),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.ge(&n2))),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.lt(&n2))),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.le(&n2))),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    TokenType::Minus => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
                        _ => bail!("Operands must be numbers."),
                    },
                    TokenType::Plus => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                        (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
                        _ => bail!(report(
                            operator,
                            "Operands must be two numbers or two strings."
                        )),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
                        _ => bail!(report(operator, "Operands must be numbers.")),
                    },
                    _ => bail!("Error"), // Unreachable
                }
            }
            Expr::Call(callee, paren, arguments) => {
                let callee = self.evaluate(callee)?;

                let mut value_args = Vec::new();
                for argument in arguments {
                    value_args.push(self.evaluate(argument)?);
                }

                // if callee != Value::Function(_) {
                //     bail!(report(paren, "Can only call functions and classes."));
                // }

                // if arguments.len() != callee.arity() {
                //     bail!(report(paren, &format!("Expected {} arguments but got {}.", callee.arity(), arguments.len())))
                // }

                match callee {
                    Value::Function(f) => {
                        if arguments.len() != f.arity() {
                            bail!("Error");
                        }
                        Ok(Value::Nil)
                    }
                    _ => bail!(report(paren, "Can only call functions and classes.")),
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Logical(left, operator, right) => {
                let left = self.evaluate(left)?;
                if operator.token_type == TokenType::Or {
                    if is_truthy(left.clone()) {
                        return Ok(left);
                    }
                } else if !is_truthy(left.clone()) {
                    return Ok(left);
                }
                self.evaluate(right)
            }
            Expr::Unary(operator, right) => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Value::Boolean(!is_truthy(right))),
                    TokenType::Minus => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => bail!(report(operator, "Operand must be a number.")),
                    },
                    _ => bail!("Error"), // Unreachable
                }
            }
            Expr::Variable(token) => self.environment.get(token),
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                self.environment = Environment::new_local(&self.environment);

                for stmt in stmts {
                    if self.execute(stmt).is_err() {
                        match self.environment.enclosing.as_ref() {
                            Some(env) => self.environment = *env.clone(),
                            None => bail!("Undefined variable"),
                        }
                    }
                }

                match self.environment.enclosing.as_ref() {
                    Some(env) => self.environment = *env.clone(),
                    None => bail!("Undefined variable"),
                }
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Function(name, _params, _body) => {
                let function = Function::new(stmt);
                self.environment
                    .define(&name.lexeme, &Value::Function(function))?;
            }
            Stmt::If(condition, then_branch, else_branch) => {
                if is_truthy(self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(b) = else_branch {
                    self.execute(b)?;
                }
            }
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(expr)?);
            }
            Stmt::Var(token, expr) => {
                let value = match expr {
                    Some(initializer) => self.evaluate(initializer)?,
                    None => Value::Nil,
                };
                self.environment.define(&token.lexeme, &value)?;
            }
            Stmt::While(condition, body) => {
                while is_truthy(self.evaluate(condition)?) {
                    self.execute(body)?;
                }
            }
        }
        Ok(())
    }
}

// Only false and nil are falsey, everything else is truthy
fn is_truthy(object: Value) -> bool {
    match object {
        Value::Boolean(b) => b,
        Value::Nil => false,
        _ => true,
    }
}

fn report(token: &Token, message: &str) -> String {
    format!("{}\n[line {}]", message, token.line)
}
