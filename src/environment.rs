use crate::result::Error;
use crate::result::Result;
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct NewEnvironment {
    enclosing: Option<Rc<RefCell<NewEnvironment>>>,
    values: RefCell<HashMap<String, Value>>,
}

impl Default for NewEnvironment {
    fn default() -> Self {
        Self::new()
    }
}

impl NewEnvironment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<NewEnvironment>>) -> Self {
        Self {
            enclosing: Some(enclosing),
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn define(&self, name: &str, value: Value) -> Result<()> {
        self.values.borrow_mut().insert(name.to_string(), value);
        Ok(())
    }

    pub fn get(&self, name: &Token) -> Result<Value> {
        if let Some(value) = self.values.borrow().get(&name.lexeme) {
            return Ok(value.clone());
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        }
        Err(Error::Runtime(
            name.clone(),
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }

    pub fn assign(&self, name: &Token, value: Value) -> Result<()> {
        if self.values.borrow().contains_key(&name.lexeme) {
            return self.define(&name.lexeme, value);
        }
        if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow_mut().assign(name, value);
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
