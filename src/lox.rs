use crate::scanner::Scanner;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};
use thiserror::Error;
use tracing::error;
use tracing::info;

#[derive(Error, Debug)]
pub enum ErrorType {
    #[error("[line {line}] Error{place:?}: {message:?}")]
    _Syntax {
        line: usize,
        place: String,
        message: String,
    },

    #[error("[line {line}] Unexpected character")]
    Lexical { line: usize },

    #[error("[line {line}] Unterminated string")]
    StringEnd { line: usize },
}

pub fn run_file(path: &str) -> Result<()> {
    let src = fs::read_to_string(path).context("Failed to read source")?;

    // TODO: If it had an error, exit.
    run(&src)
}

pub fn run_prompt() -> Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        if let Ok(0) = handle.read_line(&mut buffer) {
            println!();
            break;
        }
        run(&buffer)?;
        buffer.clear();

        // TODO: Reset an error flag.
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let mut scanner = Scanner::new(source);
    let mut tokens = scanner.scan_tokens()?;

    // For now, just print the tokens.
    for t in &mut tokens {
        info!("{}", t.get_string()?);
    }

    // TODO: Stop if there was a syntax/resolution error.
    // TODO: Add a parser, resolver and interpreter.

    Ok(())
}
