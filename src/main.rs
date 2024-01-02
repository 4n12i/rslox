use crate::lox::run_file;
use crate::lox::run_prompt;
use anyhow::bail;
use anyhow::Result;
use std::env;

mod ast_printer;
mod error;
mod expr;
mod literal;
mod lox;
mod parser;
mod scanner;
mod token;
mod token_type;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => bail!("Usage: rslox [script]"),
    }
}
