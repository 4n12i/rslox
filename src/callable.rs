use crate::expr::Expr;
use crate::interpreter::Interpreter;
use crate::literal::Literal as LoxValue;
use anyhow::Result;

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &[Expr]) -> Result<LoxValue>;
}
