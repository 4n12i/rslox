// use core::fmt;

// #[derive(Debug)]
// pub enum Value {
//     Boolean(bool),
//     Number(f64),
//     String(String),
//     Nil,
//     Function(Function),
// }

// impl fmt::Display for Value {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             Self::Number(n) => write!(f, "{n}"),
//             Self::String(s) => write!(f, "{s}"),
//             Self::Boolean(b) => write!(f, "{b}"),
//             Self::Nil => write!(f, "nil"),
//             Self::Function(_) => write!(f, "function"),
//         }
//     }
// }

// TODO: Implement From, Into method

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Function {
    pub arity: usize,
}
