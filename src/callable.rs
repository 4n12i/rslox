use crate::interpreter::Interpreter;
use crate::value::Value;
use anyhow::Result;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value>;
}
