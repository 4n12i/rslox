use crate::expr::Expr;
use crate::token::Token;
use core::fmt;

#[derive(Debug)]
pub enum Stmt {
    Block(Vec<Stmt>),
    Expression(Box<Expr>),
    If(Box<Expr>, Box<Stmt>, Option<Box<Stmt>>),
    Print(Box<Expr>),
    Var(Token, Option<Box<Expr>>),
    While(Box<Expr>, Box<Stmt>),
}

impl fmt::Display for Stmt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ast(self))
    }
}

fn format_ast(stmt: &Stmt) -> String {
    match stmt {
        Stmt::Block(stmts) => {
            let mut decls = vec![];
            for stmt in stmts {
                decls.push(format_ast(stmt));
            }
            format!("(block {})", decls.join(" "))
        }
        Stmt::Expression(expr) => format!("(; {})", expr),
        Stmt::If(condition, then_branch, else_branch) => match else_branch {
            Some(b) => format!("(if-else) {} {} {}", condition, then_branch, b),
            None => format!("(if {} {})", condition, then_branch),
        },
        Stmt::Print(value) => format!("(print {})", value),
        Stmt::Var(name, initializer) => match initializer {
            Some(i) => format!("(var {} = {})", name.lexeme, i),
            None => format!("(var {})", name.lexeme),
        },
        Stmt::While(condition, body) => format!("(while {} {})", condition, format_ast(body)),
    }
}
