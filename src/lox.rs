use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead, Read};
use thiserror::Error;

#[derive(Error, Debug)]
enum LoxError {
    #[error("[line {line}] Error{place:?}: {message:?}")]
    _SyntaxError {
        line: usize,
        place: String,
        message: String,
    },
}

pub fn main(args: &[String]) -> Result<()> {
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => bail!("Usage: rslox [script]"),
    }
}

fn run_file(path: &str) -> Result<()> {
    let mut f = File::open(path).context("Failed to open {path}")?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)
        .context("Failed to read {path}")?;

    run(&buf)
}

fn run_prompt() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        if let Ok(0) = handle.read_line(&mut buffer) {
            break;
        }
        run(&buffer)?;
        buffer.clear();
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let token: Vec<&str> = source.split_whitespace().collect();
    for t in token {
        println!("{t}");
    }

    Ok(())
}
