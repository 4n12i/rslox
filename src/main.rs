use anyhow::bail;
use anyhow::Result;
use lox::Lox;
use std::env;

mod environment;
mod expr;
mod interpreter;
mod literal;
mod lox;
mod parser;
mod scanner;
mod stmt;
mod token;
mod token_type;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => Lox::run_prompt(),
        1 => Lox::run_file(&args[0]),
        _ => bail!("Usage: rslox [script]"),
    }
}
