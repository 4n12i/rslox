extern crate rslox;

use rslox::lox::Lox;
use rslox::result::Error;
use rslox::result::Result;
use std::env;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().skip(1).collect();
    match args.len() {
        0 => Lox::run_prompt(),
        1 => Lox::run_file(&args[0]),
        _ => Err(Error::Usage),
    }
}
