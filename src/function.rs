use crate::callable::Callable;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::value::Value;
use anyhow::Result;
use core::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Function {
    name: Token,
    parameters: Vec<Token>,
    body: Stmt,
}

impl Function {
    pub fn _new(stmt: &Stmt) -> Self {
        match stmt {
            Stmt::Function(name, params, body) => Self {
                name: name.clone(),
                parameters: params.clone(),
                body: *body.clone(),
            },
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name.lexeme)
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.parameters.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value> {
        let mut environment = Environment::new_local(&interpreter.globals);
        for (param, arg) in self.parameters.iter().zip(arguments) {
            environment.define(&param.lexeme, arg)?;
        }

        // Execute block statement
        interpreter.execute(&self.body)?;
        Ok(Value::Nil)
    }
}
