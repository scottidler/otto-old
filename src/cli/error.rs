#![allow(unused_imports, unused_variables, dead_code)]
use anyhow;

use std::{
    fmt,
    result,
};
use std::error::Error;
use crate::cli::token::Token;

pub type Result<T> = result::Result<T, ParseError>;

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken(Token),
    ProtectedNotChoice(Token, Vec<String>),
    Custom(String),
}

impl Error for ParseError {
    fn description(&self) -> &str {
        match *self {
            ParseError::UnexpectedToken(ref token) => "unexpected token",
            ParseError::ProtectedNotChoice(ref token, ref choices) => "protected not choice",
            ParseError::Custom(ref err) => err,
        }
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseError::UnexpectedToken(ref token) => write!(fmtr, "parse error: unexpected token={:?}", token),
            ParseError::ProtectedNotChoice(ref token, ref choices) => write!(fmtr, "parse error: protected={} not one of choices={:?}", token, choices),
            ParseError::Custom(ref err) => write!(fmtr, "parse error: custom error {}", err),
        }
    }
}

impl From<anyhow::Error> for ParseError {
    fn from(error: anyhow::Error) -> Self {
        ParseError::Custom(error.to_string())
    }
}

use crate::cfg::error::ConfigError;

impl From<ConfigError> for ParseError {
    fn from(error: ConfigError) -> Self {
        ParseError::Custom(error.to_string())
    }
}
