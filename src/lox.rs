use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::scanner::Scanner;
use anyhow::Context;
use anyhow::Result;
use std::fs;
use std::io::BufRead;
use std::io::Write;
use std::io::{self};

pub struct Lox {
    interpreter: Interpreter,
}

impl Lox {
    fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn run_file(path: &str) -> Result<()> {
        let mut lox = Self::new();
        let src = fs::read_to_string(path).context("Failed to read a source file")?;
        lox.run(&src)
    }

    pub fn run_prompt() -> Result<()> {
        let mut lox = Self::new();
        let mut handle = io::stdin().lock();
        let mut buffer = String::new();

        loop {
            print!("> ");
            io::stdout().flush()?;
            if let Ok(0) = handle.read_line(&mut buffer) {
                println!();
                break;
            }
            if let Err(e) = lox.run(&buffer) {
                eprintln!("{e}"); // Reset an error
            }
            buffer.clear();
        }

        Ok(())
    }

    fn run(&mut self, source: &str) -> Result<()> {
        let tokens = Scanner::new(source).run()?;
        let stmts = Parser::new(tokens).run()?;
        self.interpreter.run(&stmts)?;

        Ok(())
    }
}
