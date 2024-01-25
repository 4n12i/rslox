use crate::token::Token;
use crate::value::Value;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Assign(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Grouping(Box<Expr>),
    Literal(Value),
    Logical(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Variable(Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format_ast(self))
    }
}

fn format_ast(expr: &Expr) -> String {
    match expr {
        Expr::Assign(name, value) => format!("(= {} {})", name.lexeme, value),
        Expr::Binary(left, operator, right) => format!(
            "({} {} {})",
            operator.lexeme,
            format_ast(left),
            format_ast(right)
        ),
        Expr::Call(callee, _paren, arguments) => {
            let mut str_args = vec![];
            for argument in arguments {
                str_args.push(format_ast(argument));
            }
            format!("(call {} {})", callee, str_args.join(" "))
        }
        Expr::Grouping(expr) => format!("(group {})", format_ast(expr)),
        Expr::Literal(value) => value.to_string(),
        Expr::Logical(left, operator, right) => format!(
            "({} {} {})",
            format_ast(left),
            operator.lexeme,
            format_ast(right)
        ),
        Expr::Unary(operator, right) => format!("({} {})", operator.lexeme, format_ast(right)),
        Expr::Variable(name) => name.lexeme.clone(),
    }
}

#[cfg(test)]
mod tests {
    use crate::literal::Literal;
    use crate::token_type::TokenType;

    use super::*;
    #[test]
    fn print_ast() {
        let e = format_ast(&Expr::Binary(
            Box::new(Expr::Unary(
                Token::new(TokenType::Minus, "-", Literal::Nil, 1),
                Box::new(Expr::Literal(Value::Number(123f64))),
            )),
            Token::new(TokenType::Star, "*", Literal::Nil, 1),
            Box::new(Expr::Grouping(Box::new(Expr::Literal(Value::Number(
                45.67f64,
            ))))),
        ));
        println!("{e}");
    }
}
