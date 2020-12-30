use anyhow::{anyhow,Error,Result};
use std::collections::HashMap;

use super::token::Token;
use super::ast::AST;
use crate::cfg::spec::{
    Spec,
    Otto,
    Task,
    Param,
};

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


//const OTTO: Token = Token::KWD("otto".to_string());

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
    /*
    fn task_names(&self) -> Vec<String> {
        self.spec.otto.tasks.keys().cloned().collect()
    }
    */
    fn task_names(&self) -> Vec<String> {
        self.spec.otto.tasks.iter().map(|t| t.name.clone()).collect()
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
        //let ast = self.parse_otto(&self.spec.otto)?;
        Ok(&self.spec.otto)
    }
    fn parse_otto(&mut self, otto: &Otto) -> Result<AST> {
        let otto_tok = Token::KWD("otto".to_string());
        let mut asts: Vec<AST> = vec![];
        while let Some(token) = self.peek() {
            let ast = match token {
                Token::KWD(kwd) => {
                    /*
                    let task = otto.tasks.get(&kwd).unwrap();
                    self.parse_task(*task)?
                    */
                    return Err(anyhow!("parse_test don't support task yet"))
                }
                Token::SHT(sht) => {
                    /*
                    let param = otto.params.get_short(sht).expect("parse_otto: expected param for short {}", sht);
                    self.parse_param(param)?
                    */
                    return Err(anyhow!("parse_test don't support short yet"))
                },
                Token::LNG(lng) => {
                    /*
                    let param = otto.params.get_long(lng).expect("parse_otto: expected param for long {}", lng);
                    self.parse_param(param)?
                    */
                    return Err(anyhow!("parse_test don't support long yet"))
                },
                Token::VAL(val) => {
                    // FIXME: this is where we must track postional variables
                    return Err(anyhow!("parse_test don't support positional yet"))
                },
                //Token::BLT(_) => self.parse_builtin()?, FIXME: support for builtins like 'help'
                _ => return Err(anyhow!("something"))
            };
            asts.push(ast);
        }
        Ok(AST::Cmd(otto_tok, asts))
    }
    fn parse_builtin(&mut self) -> Result<AST> {
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    fn parse_task(&mut self, task: Task) -> Result<AST> {
        let task_tok = self.next().expect("parse_task expected Token, got None");
        let mut asts: Vec<AST> = vec![];
        while let Some(token) = self.peek() {
            let ast = match token {
                Token::SHT(sht) => {
                    /*
                    let param = task.params.get_short(sht)?;
                    self.parse_param(param)?
                    */
                    return Err(anyhow!("parse_test don't support short yet"))
                },
                Token::LNG(lng) => {
                    /*
                    let param = task.params.get_long(lng)?;
                    self.parse_param(param)?
                    */
                    return Err(anyhow!("parse_test don't support long yet"))
                },
                Token::VAL(val) => {
                    // FIXME: this is where we must track postional variables
                    return Err(anyhow!("parse_test don't support positional yet"))
                },
                _ => return Err(anyhow!("parse_text expected SHT, LNG or VAL token, got {}", token)),
            };
            asts.push(ast);
        }
        Ok(AST::Cmd(task_tok, asts))
    }
    fn parse_param(&mut self, param: Param) -> Result<AST> {
        let arg = self.next().expect("parse_param expected Token got None");
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    pub fn peek(&mut self) -> Option<Token> {
        let token = match self.tokens.get(self.index) {
            Some(token) => token.clone(),
            None => return None,
        };
        Some(token)
    }
    pub fn next(&mut self) -> Option<Token> {
        let token = self.peek()?;
        self.index += 1;
        Some(token)
    }
}
