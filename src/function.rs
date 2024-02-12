use crate::callable::Callable;
use crate::environment::Environment;
use crate::interpreter::Interpreter;
use crate::result::Error;
use crate::result::Result;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::cmp;
use std::fmt;
use std::rc::Rc;

// User-defined function
#[derive(Clone, Debug)]
pub struct LoxFunction {
    name: String,
    params: Vec<Token>,
    body: Vec<Stmt>,
    closure: Rc<RefCell<Environment>>,
}

impl LoxFunction {
    pub fn new(
        name: &Token,
        params: &[Token],
        body: &Stmt,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        let block = match body {
            Stmt::Block(ref stmts) => stmts,
            _ => unreachable!(),
        };
        Self {
            name: name.lexeme.clone(),
            params: params.to_vec(),
            body: block.to_vec(),
            closure,
        }
    }
}

impl Callable for LoxFunction {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value> {
        let environment = Environment::with_enclosing(Rc::clone(&self.closure));

        let previous = Rc::clone(&interpreter.environment);

        for (param, arg) in self.params.iter().zip(arguments) {
            environment.define(&param.lexeme, arg.clone())?;
        }

        // Execute block statement
        interpreter.environment = Rc::new(RefCell::new(environment));
        for stmt in &self.body {
            if let Err(error) = interpreter.execute(stmt) {
                interpreter.environment = previous;
                match error {
                    Error::Return(value) => return Ok(value),
                    _ => return Err(error),
                }
            }
        }
        interpreter.environment = previous;
        Ok(Value::Nil)
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<fn {}>", self.name)
    }
}

impl cmp::PartialEq for LoxFunction {
    fn eq(&self, _: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd for LoxFunction {
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        todo!()
    }
}

// Primitive function
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct NativeFunction {
    arity: usize,
    function: fn(&mut Interpreter, &[Value]) -> Result<Value>,
}

impl NativeFunction {
    pub fn new(arity: usize, function: fn(&mut Interpreter, &[Value]) -> Result<Value>) -> Self {
        Self { arity, function }
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value> {
        (self.function)(interpreter, arguments)
    }
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native fn>")
    }
}
