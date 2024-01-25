use crate::interpreter::Interpreter;
use crate::value::Value;
// use anyhow::Result;
use crate::result::Result;

pub trait Callable {
    // Number of arguments
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value>;
}
