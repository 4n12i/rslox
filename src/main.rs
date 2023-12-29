use anyhow::Result;
use std::env;

mod lox;
mod token;
mod token_type;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().skip(1).collect();

    lox::main(&args)
}
