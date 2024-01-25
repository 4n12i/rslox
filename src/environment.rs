use crate::token::Token;
use crate::value::Value;
// use anyhow::bail;
// use anyhow::Result;
use crate::result::{Error, Result};
use std::collections::HashMap;

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

    // Definition of variables. You can also redifine existing variables.
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
                // bail!("Undefined variable '{}'", name.lexeme)
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
        // bail!("Undefined variable '{}'", name.lexeme)
        Err(Error::Runtime(
            name.clone(),
            format!("Undefined variable '{}'", name.lexeme),
        ))
    }
}
