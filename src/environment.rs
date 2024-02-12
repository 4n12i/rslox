use crate::result::Error;
use crate::result::Result;
use crate::token::Token;
use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Environment {
    enclosing: Option<Rc<RefCell<Environment>>>,
    values: RefCell<HashMap<String, Value>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_enclosing(enclosing: Rc<RefCell<Environment>>) -> Self {
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
