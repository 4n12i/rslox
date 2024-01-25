use crate::interpreter::Interpreter;
use crate::result::Result;
use crate::value::Value;

pub trait Callable {
    // Number of arguments
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Value]) -> Result<Value>;
}
