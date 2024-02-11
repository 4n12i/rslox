use crate::callable::Callable;
use crate::environment::NewEnvironment;
use crate::interpreter::Interpreter;
use crate::result::Error;
use crate::result::Result;
use crate::stmt::Stmt;
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Function {
    declaration: Declaration,
    // initializer: bool,
    // closure: Environment,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
enum Declaration {
    UserDefined(Token, Vec<Token>, Box<Stmt>), // User-defined function
    Primitive(fn(&mut Interpreter, &[Value]) -> Result<Value>, usize),
}

// Convert Stmt::Function to Declaration::UserDefined
impl From<Stmt> for Declaration {
    fn from(value: Stmt) -> Self {
        match value {
            Stmt::Function(name, params, body) => Declaration::UserDefined(name, params, body),
            _ => unreachable!(),
        }
    }
}

impl Function {
    pub fn new(stmt: &Stmt) -> Self {
        Self {
            declaration: stmt.clone().into(),
        }
    }

    pub fn new_primitive(
        function: fn(&mut Interpreter, &[Value]) -> Result<Value>,
        arity: usize,
    ) -> Self {
        Self {
            declaration: Declaration::Primitive(function, arity),
        }
    }
}

impl fmt::Display for Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.declaration {
            Declaration::UserDefined(name, _, _) => write!(f, "<fn {}>", name.lexeme),
            Declaration::Primitive(_, _) => write!(f, "<native fn>"),
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        match &self.declaration {
            Declaration::UserDefined(_, params, _) => params.len(),
            Declaration::Primitive(_, arity) => *arity,
        }
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value> {
        match &self.declaration {
            Declaration::UserDefined(_, params, body) => {
                let environment = NewEnvironment::with_enclosing(Rc::clone(&interpreter.globals));
                let previous = Rc::clone(&interpreter.environment);

                for (param, arg) in params.iter().zip(arguments) {
                    environment.define(&param.lexeme, arg.clone())?;
                }

                // Execute block statement
                match **body {
                    Stmt::Block(ref stmts) => {
                        interpreter.environment = Rc::new(RefCell::new(environment));
                        for stmt in stmts {
                            if let Err(e) = interpreter.execute(stmt) {
                                interpreter.environment = previous;
                                match e {
                                    Error::Return(v) => return Ok(v),
                                    _ => return Err(e),
                                }
                            }
                        }
                        interpreter.environment = previous;
                        Ok(Value::Nil)
                    }
                    _ => unreachable!(),
                }
            }
            Declaration::Primitive(function, _) => function(interpreter, arguments),
        }
    }
}
