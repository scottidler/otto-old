use anyhow::{
    anyhow,
    Result
};

use super::token::Token;
use super::ast::AST;
use crate::cfg::spec::{
    Spec,
    Otto,
    Task,
    Param,
    Nargs,
    Value,
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
        Self {
            spec: spec,
            tasks: vec!["otto".to_string()],
            tokens: vec![],
            index: 0,
        }
    }
    fn task_names(&self) -> Vec<String> {
        self.spec.otto.tasks.values().map(|t| t.name.clone()).collect()
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
            else if arg.starts_with("-") {
                tokens.push(Token::ARG(arg.to_string()))
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
    pub fn parse(&mut self, args: &Vec<String>) -> Result<Otto> {
        self.tokens = self.tokenize(args);
        let mut otto = self.spec.otto.clone();
        while let Ok(token) = self.peek() {
            println!("parse: token={:?}", token);
            match token {
                Token::KWD(kwd) => {
                    self.next()?;
                    let task = otto.get_task(&kwd)?;
                    let task2 = self.parse_task(task)?;
                    otto.set_task(task2)?;
                }
                Token::ARG(arg) => {
                    let param = otto.get_param_from_flag(&arg)?;
                    let param2 = self.parse_param(param)?;
                    otto.set_param(param2)?;
                },
                Token::VAL(val) => {
                    // FIXME: this is where we must track postional variables
                    return Err(anyhow!("parse_test don't support positional yet; val={}", val))
                },
                //Token::BLT(_) => self.parse_builtin()?, FIXME: support for builtins like 'help'
                _ => return Err(anyhow!("unexpected error on token={:?}", token))
            };
        }
       Ok(otto)
    }
    fn parse_builtin(&mut self) -> Result<AST> {
        Ok(AST::Atom(Token::VAL("hi".to_string())))
    }
    fn parse_task(&mut self, task: &Task) -> Result<Task> {
        let mut task = task.clone();
        task.selected = true;
        while let Ok(token) = self.peek() {
            println!("parse: token={:?}", token);
            match token {
                Token::KWD(kwd) => {
                    break;
                }
                Token::ARG(arg) => {
                    self.next()?;
                    let param = task.get_param_from_flag(&arg)?;
                    let param2 = self.parse_param(param)?;
                    task.set_param(param2)?;
                },
                Token::VAL(val) => {
                    // FIXME: this is where we must track postional variables
                    return Err(anyhow!("parse_test don't support positional yet; val={}", val))
                },
                //Token::BLT(_) => self.parse_builtin()?, FIXME: support for builtins like 'help'
                _ => return Err(anyhow!("unexpected error on token={:?}", token))
            };
        }
        Ok(task)
    }
    fn parse_param(&mut self, param: &Param) -> Result<Param> {
        println!("before parse_param: param={:?}", param);
        let mut param = param.clone();
        match param.nargs {
            Nargs::One => self.parse_one(&mut param)?,
            Nargs::Zero => self.parse_zero(&mut param)?,
            Nargs::OneOrZero => self.parse_one_or_zero(&mut param)?,
            Nargs::OneOrMore => self.parse_one_or_more(&mut param)?,
            Nargs::ZeroOrMore => self.parse_zero_or_more(&mut param)?,
            Nargs::Range(min, max) => self.parse_range(&mut param, min, max)?,
        };
        println!("after parse_param: param={:?}", param);
        Ok(param)
    }
    pub fn parse_one(&mut self, param: &mut Param) -> Result<()> {
        let token = self.peek()?;
        println!("parse_one: token={:#?}", token);
        match token {
            Token::VAL(val) => {
                self.next();
                param.value = Value::Item(val.to_owned());
            }
            _ => println!("something else"),
        }
        Ok(())
    }
    pub fn parse_zero(&mut self, param: &mut Param) -> Result<()> {
        Ok(())
    }
    pub fn parse_one_or_zero(&mut self, param: &mut Param) -> Result<()> {
        Ok(())
    }
    pub fn parse_one_or_more(&mut self, param: &mut Param) -> Result<()> {
        Ok(())
    }
    pub fn parse_zero_or_more(&mut self, param: &mut Param) -> Result<()> {
        Ok(())
    }
    pub fn parse_range(&mut self, param: &mut Param, min: usize, max: usize) -> Result<()> {
        Ok(())
    }
    pub fn peek(&mut self) -> Result<Token> {
        match self.tokens.get(self.index) {
            Some(token) => Ok(token.clone()),
            None => Err(anyhow!("peek: failed to get next token")),
        }
    }
    pub fn next(&mut self) -> Result<Token> {
        let token = self.peek()?;
        self.index += 1;
        Ok(token)
    }
}
