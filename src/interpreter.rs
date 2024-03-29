use crate::callable::Callable;
use crate::environment::Environment;
use crate::expr::Expr;
use crate::function::LoxFunction;
use crate::function::NativeFunction;
use crate::result::Error;
use crate::result::Result;
use crate::stmt::Stmt;
use crate::token_type::TokenType;
use crate::value::Value;
use std::cell::RefCell;
use std::default::Default;
use std::rc::Rc;

pub struct Interpreter {
    pub globals: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

impl Interpreter {
    pub fn new() -> Self {
        let environment = Environment::new();
        let globals = Rc::new(RefCell::new(environment));

        // Define a primitive function
        fn clock(_: &mut Interpreter, _: &[Value]) -> Result<Value> {
            use std::time::SystemTime;
            use std::time::UNIX_EPOCH;

            let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
            let milliseconds = current_time.as_millis() as f64;
            Ok(Value::Number(milliseconds / 1000.0))
        }
        let function = NativeFunction::new(0, clock);
        globals
            .borrow_mut()
            .define("clock", Value::NativeFunction(function))
            .expect("Failed to define a primitive function.");

        Self {
            globals: Rc::clone(&globals),
            environment: globals,
        }
    }

    pub fn run(&mut self, statements: &[Stmt]) -> Result<()> {
        for statement in statements {
            self.execute(statement)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value> {
        match expr {
            Expr::Assign(name, value) => {
                let value = self.evaluate(value)?;
                self.environment.borrow_mut().assign(name, value.clone())?;
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
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::GreaterEqual => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.ge(&n2))),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::Less => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.lt(&n2))),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::LessEqual => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Boolean(n1.le(&n2))),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::Minus => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::Plus => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
                        (Value::String(s1), Value::String(s2)) => Ok(Value::String(s1 + &s2)),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be two numbers or two strings.".to_string(),
                        )),
                    },
                    TokenType::Slash => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    TokenType::Star => match (left, right) {
                        (Value::Number(n1), Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operands must be numbers.".to_string(),
                        )),
                    },
                    _ => unreachable!(),
                }
            }
            Expr::Call(callee, paren, arguments) => {
                let callee = self.evaluate(callee)?;

                let mut value_args = Vec::new();
                for argument in arguments {
                    value_args.push(self.evaluate(argument)?);
                }

                match callee {
                    Value::LoxFunction(f) => {
                        if arguments.len() != f.arity() {
                            return Err(Error::Runtime(
                                paren.clone(),
                                format!(
                                    "Expected {} arguments but get {}.",
                                    f.arity(),
                                    arguments.len()
                                ),
                            ));
                        }
                        f.call(self, &value_args)
                    }
                    _ => Err(Error::Runtime(
                        paren.clone(),
                        "Can only call functions and classes.".to_string(),
                    )),
                }
            }
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Literal(value) => Ok(value.clone()),
            Expr::Logical(left, operator, right) => {
                let left = self.evaluate(left)?;
                if operator.token_type == TokenType::Or {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                } else if !left.is_truthy() {
                    return Ok(left);
                }
                self.evaluate(right)
            }
            Expr::Unary(operator, right) => {
                let right = self.evaluate(right)?;
                match operator.token_type {
                    TokenType::Bang => Ok(Value::Boolean(!right.is_truthy())),
                    TokenType::Minus => match right {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(Error::Runtime(
                            operator.clone(),
                            "Operand must be a number.".to_string(),
                        )),
                    },
                    _ => unreachable!(),
                }
            }
            Expr::Variable(token) => self.environment.borrow().get(token),
        }
    }

    pub fn execute(&mut self, stmt: &Stmt) -> Result<()> {
        match stmt {
            Stmt::Block(stmts) => {
                let previous = Rc::clone(&self.environment);
                self.environment = Rc::new(RefCell::new(Environment::with_enclosing(Rc::clone(
                    &self.environment,
                ))));

                for stmt in stmts {
                    if let Err(e) = self.execute(stmt) {
                        self.environment = previous;
                        return Err(e);
                    }
                }

                self.environment = previous;
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Function(name, params, body) => {
                let function = LoxFunction::new(name, params, body, Rc::clone(&self.environment));
                self.environment
                    .borrow_mut()
                    .define(&name.lexeme, Value::LoxFunction(function))?;
            }
            Stmt::If(condition, then_branch, else_branch) => {
                if self.evaluate(condition)?.is_truthy() {
                    self.execute(then_branch)?;
                } else if let Some(b) = else_branch {
                    self.execute(b)?;
                }
            }
            Stmt::Print(expr) => {
                println!("{}", self.evaluate(expr)?);
            }
            Stmt::Return(_keyword, value) => {
                let value = match value {
                    Some(v) => self.evaluate(v)?,
                    None => Value::Nil,
                };
                return Err(Error::Return(value));
            }
            Stmt::Var(token, expr) => {
                let value = match expr {
                    Some(initializer) => self.evaluate(initializer)?,
                    None => Value::Nil,
                };
                self.environment.borrow_mut().define(&token.lexeme, value)?;
            }
            Stmt::While(condition, body) => {
                while self.evaluate(condition)?.is_truthy() {
                    self.execute(body)?;
                }
            }
        }
        Ok(())
    }
}
