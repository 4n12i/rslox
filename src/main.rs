use anyhow::bail;
use anyhow::Result;
use std::env;

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
        0 => lox::run_prompt(),
        1 => lox::run_file(&args[0]),
        _ => bail!("Usage: rslox [script]"),
    }
}
