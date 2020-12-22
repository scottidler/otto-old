use anyhow::Result;
use std::collections::HashMap;

use super::token::Token;
use crate::cfg::spec::{Spec, Otto, Task};

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub spec: Spec,
    pub tasks: Vec<String>,
    pub tokens: Vec<Token>,
}

//type ParamMap = HashMap<String, String>;
//type TaskMap = HashMap<String, ParamMap>;

impl Parser {
    pub fn new(mut spec: Spec) -> Self {
        //spec.otto.selected = true;
        Self {
            spec: spec,
            tasks: vec!["otto".to_string()],
            tokens: vec![],
        }
    }
    pub fn parse(mut self, args: Vec<String>) -> Result<Otto> {
        Ok(self.spec.otto)
    }
    fn parse_task(mut self, token: Token, mut context: Task) {}
    fn parse_flag(mut self, token: Token) {}
    fn parse_positional(mut self, token: Token, count: u32) {}
    fn next(&mut self) -> &Token {
        unimplemented!()
    }
    fn peek(&mut self) -> &Token {
        unimplemented!()
    }
}
