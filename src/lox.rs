use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};

pub fn run_file(path: &str) -> Result<()> {
    let src = fs::read_to_string(path).context("Failed to read a source file")?;
    run(&src)
}

pub fn run_prompt() -> Result<()> {
    let mut buffer = String::new();
    let mut handle = io::stdin().lock();

    loop {
        print!("> ");
        io::stdout().flush()?;
        if let Ok(0) = handle.read_line(&mut buffer) {
            println!();
            break;
        }
        if let Err(e) = run(&buffer) {
            eprintln!("{e}"); // Reset an error
        }
        buffer.clear();
    }

    Ok(())
}

fn run(source: &str) -> Result<()> {
    let tokens = Scanner::new(source).run()?;
    let expr = Parser::new(tokens).run()?;
    Interpreter::run(&expr)?;

    Ok(())
}
