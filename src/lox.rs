use crate::ast_printer::format_ast;
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};
use tracing::info;

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

    let mut parser = Parser::new(tokens);
    let expr = parser.run()?;
    info!("{}", format_ast(expr)?);

    // TODO: Stop if there was a syntax/resolution error.

    Ok(())
}
