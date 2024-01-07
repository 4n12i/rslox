use crate::literal::Literal as LoxValue;
use crate::token::Token;
use anyhow::{bail, Result};
use std::collections::HashMap;

pub struct Environment {
    pub values: HashMap<String, LoxValue>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    // Definition of variables. You can also redifine existing variables.
    pub fn define(&mut self, name: &str, value: &LoxValue) -> Result<()> {
        self.values.insert(name.to_string(), value.clone());
        Ok(())
    }

    pub fn get(&mut self, name: &Token) -> Result<LoxValue> {
        match self.values.get(&name.lexeme) {
            Some(n) => Ok(n.clone()),
            None => bail!("Undefined variable '{}'", name.lexeme),
        }
    }
}
