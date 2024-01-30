extern crate rslox;

use rslox::lox::Lox;
use rslox::result::Error;
use rslox::result::Result;
use std::env;
use std::process::exit;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let args: Vec<String> = env::args().skip(1).collect();
    let result = match args.len() {
        0 => Lox::run_prompt(),
        1 => Lox::run_file(&args[0]),
        _ => Err(Error::Usage),
    };

    match result {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
        }
    }
}
