use anyhow::Result;
use std::env;
use tracing::info;

mod lox;

fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let args: Vec<String> = env::args().skip(1).collect();
    info!("ARGS: {}", args.len());

    lox::main(&args)
}
