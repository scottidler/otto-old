use std::collections::HashMap;

use super::token::Token;
use crate::cfg::spec::Otto;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub otto: Otto,
}

type ParamMap = HashMap<String, String>;
type TaskMap = HashMap<String, ParamMap>;

impl Parser {
    pub fn new(otto: Otto) -> Self {
        Self {
            otto: otto,
        }
    }
    pub fn parse(&self, tokens: Vec<Token>) -> TaskMap {
        let parsed = TaskMap::new();
        parsed
    }
}
