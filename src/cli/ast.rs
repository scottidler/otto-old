use std::fmt;
use super::token::Token;

pub enum AST {
    Atom(Token),
    Array(Vec<Token>),
    Assign(Token, Box<AST>),
    Cmd(Token, Vec<AST>),
}

impl fmt::Display for AST {
    fn fmt(&self, fmtr: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AST::Atom(token) => write!(fmtr, "{}", token),
            //AST::Array(tokens) => write!(fmtr, "[{}]", (*tokens).connect(" ")),
            AST::Array(tokens) => {
                write!(fmtr, "[")?;
                for token in tokens {
                    write!(fmtr, "{} ", token)?;
                }
                write!(fmtr, "]")
            }
            AST::Assign(token, ast) => write!(fmtr, "{}={}", token, ast),
            AST::Cmd(token, asts) => {
                write!(fmtr, "({}", token)?;
                for ast in asts {
                    write!(fmtr, " {}", ast)?;
                }
                write!(fmtr, ")")
            }
        }
    }
}