use crate::lox::run_file;
use crate::lox::run_prompt;
use anyhow::bail;
use anyhow::Result;
use std::env;

mod lox;
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
