use anyhow::{anyhow,Error,Result};
use std::collections::HashMap;

use super::token::Token;
use super::ast::AST;
use crate::cfg::spec::{Spec, Otto, Task};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parser {
    pub spec: Spec,
    pub tasks: Vec<String>,
    pub tokens: Vec<Token>,
    index: usize,
}

//type ParamMap = HashMap<String, String>;
//type TaskMap = HashMap<String, ParamMap>;

impl Parser {
    pub fn new(spec: Spec) -> Self {
        //spec.otto.selected = true;
        Self {
            spec: spec,
            tasks: vec!["otto".to_string()],
            tokens: vec![],
            index: 0,
        }
    }
    fn task_names(&self) -> Vec<String> {
        self.spec.otto.tasks.keys().cloned().collect()
    }
    fn builtin_names(&self) -> Vec<String> {
        vec!["help".to_string()]
    }
    fn tokenize(&self, args: &Vec<String>) -> Vec<Token> {
        let mut tokens = vec![];
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            if arg == "--" {
                let mut rem = vec![];
                while let Some(arg) = iter.next() {
                    rem.push(arg.to_string())
                }
                tokens.push(Token::REM(rem))
            }
            else if arg.starts_with("--") {
                tokens.push(Token::LNG(arg.to_string()))
            }
            else if arg.starts_with("-") {
                tokens.push(Token::SHT(arg.to_string()))
            }
            else if self.builtin_names().iter().any(|name| name == arg){
                tokens.push(Token::BLT(arg.to_string()))
            }
            else if self.task_names().iter().any(|name| name == arg) {
                tokens.push(Token::KWD(arg.to_string()))
            }
            else {
                tokens.push(Token::VAL(arg.to_string()))
            }
        }
        tokens
    }
    pub fn parse(&mut self, args: &Vec<String>) -> Result<&Otto> {
        self.tokens = self.tokenize(args);
        let ast = self.parse_otto()?;
        Ok(&self.spec.otto)
    }
    fn parse_otto(&mut self) -> Result<AST> {
        let mut asts: Vec<AST> = vec![];
        while let Some(token) = self.peek() {
            let ast = match token {
                Token::BLT(_) => self.parse_builtin()?,
                Token::KWD(_) => self.parse_task()?,
                Token::SHT(_) | Token::LNG(_) => self.parse_param()?,
                Token::VAL(_) | Token::REM(_) => return Err(anyhow!("something"))
            };
            asts.push(ast);
        }
        Ok(AST::Cmd(Token::KWD("otto".to_string()), asts))
    }
    fn parse_builtin(&mut self) -> Result<AST> {
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    fn parse_task(&mut self) -> Result<AST> {
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    fn parse_param(&mut self) -> Result<AST> {
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    pub fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index)
    }
    pub fn next(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.index);
        self.index += 1;
        token
    }
}
