use std::collections::HashMap;

use super::token::Token;
use crate::cfg::spec::Task;

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub otto: Task,
}

type ParamMap = HashMap<String, String>;
type TaskMap = HashMap<String, ParamMap>;

impl Parser {
    pub fn new(otto: Task) -> Self {
        Self {
            otto: otto,
        }
    }
    pub fn parse(&self, tokens: Vec<Token>) -> TaskMap {
        let parsed = TaskMap::new();
        parsed
    }
}
