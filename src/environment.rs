use crate::result::{Error, Result};
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[allow(dead_code)]
pub struct NewNode {
    enclosing: Option<Rc<RefCell<NewNode>>>, // Pointer to parent environment
    values: HashMap<String, Value>,
}

#[allow(dead_code)]
pub struct NewEnvironment {
    node: Rc<RefCell<NewNode>>,
}

#[allow(dead_code)]
impl NewNode {
    fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    fn with_enclosing(enclosing: Rc<RefCell<NewNode>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: HashMap::new(),
        }
    }

    // Definition of variables. You can also redefine existing variables.
    fn define(&mut self, name: &str, value: Value) -> bool {
        self.values.insert(name.to_string(), value).is_some()
    }

    fn get(&self, name: &Token) -> Result<Value> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(ref e) = self.enclosing {
                    return e.borrow_mut().get(name);
                }
                Err(Error::Runtime(
                    name.clone(),
                    format!("Undefined variable '{}'", name.lexeme),
                ))
            }
        }
    }

    fn assign(&mut self, name: &Token, value: Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        if let Some(ref mut e) = self.enclosing {
            return e.borrow_mut().assign(name, value);
        }
        Err(Error::Runtime(
            name.clone(),
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
}

impl Environment {
    // For global scope
    pub fn new_global() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    // For local scope
    pub fn new_local(enclosing: &Environment) -> Self {
        Self {
            enclosing: Some(Box::new(enclosing.clone())),
            values: HashMap::new(),
        }
    }

    // Definition of variables. You can also redefine existing variables.
    pub fn define(&mut self, name: &str, value: &Value) -> Result<()> {
        self.values.insert(name.to_string(), value.clone());
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        match self.values.get(&name.lexeme) {
            Some(value) => Ok(value.clone()),
            None => {
                if let Some(ref e) = self.enclosing {
                    return e.get(name);
                }
                Err(Error::Runtime(
                    name.clone(),
                    format!("Undefined variable '{}'", name.lexeme),
                ))
            }
        }
    }

    pub fn assign(&mut self, name: &Token, value: &Value) -> Result<()> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value.clone());
            return Ok(());
        }
        if let Some(ref mut e) = self.enclosing {
            return e.assign(name, value);
        }
        Err(Error::Runtime(
            name.clone(),
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }
}
