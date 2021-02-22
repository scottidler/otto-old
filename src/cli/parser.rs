use super::error::{
    ParseError,
    Result
};
use super::token::Token;
use crate::cfg::spec::{
    Spec,
    Otto,
    Task,
    Param,
    Nargs,
    Value,
};

use std::collections::HashMap;

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
        self.spec.otto.tasks.values().map(|t| t.name.to_owned()).collect()
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
            else if arg.contains("=") {
                tokens.push(Token::KVP(arg.to_string()))
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
        tokens.push(Token::EOF);
        tokens
    }
    pub fn parse(&mut self, args: &Vec<String>) -> Result<Otto> {
        self.tokens = self.tokenize(args);
        let mut otto = self.spec.otto.to_owned();
        while let Ok(token) = self.peek() {
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
                    return Err(ParseError::Custom(format!("parse_test doesn't support positional yet; val={}", val)))
                },
                Token::EOF => break,
                //Token::BLT(_) => self.parse_builtin()?, FIXME: support for builtins like 'help'
                _ => return Err(ParseError::UnexpectedToken(token.to_owned()))
            };
        }
       Ok(otto)
    }
    fn parse_builtin(&mut self) -> Result<Task> {
        let task = Task::new(
            "help".to_string(),
            Some("help".to_string()),
            vec![],
            vec![],
            HashMap::new(),
            None,
            true,
        );
        Ok(task)
    }
    fn parse_task(&mut self, task: &Task) -> Result<Task> {
        let mut task = task.to_owned();
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
                    return Err(ParseError::Custom(format!("parse_test doesn't support positional yet; val={}", val)))
                },
                Token::EOF => break,
                //Token::BLT(_) => self.parse_builtin()?, FIXME: support for builtins like 'help'
                _ => return Err(ParseError::UnexpectedToken(token.to_owned()))
            };
        }
        Ok(task)
    }
    fn parse_param(&mut self, param: &Param) -> Result<Param> {
        println!("parse_param:");
        let mut param = param.to_owned();
        match param.nargs {
            Nargs::One => self.parse_one(&mut param)?,
            Nargs::Zero => self.parse_zero(&mut param)?,
            Nargs::OneOrZero => self.parse_one_or_zero(&mut param)?,
            Nargs::OneOrMore => self.parse_one_or_more(&mut param)?,
            Nargs::ZeroOrMore => self.parse_zero_or_more(&mut param)?,
            Nargs::Range(min, max) => self.parse_range(&mut param, min, max)?,
        };
        Ok(param)
    }
    fn parse_many(&mut self, param: &mut Param, at_least_one: bool) -> Result<()> {
        let mut vs = vec![];
        while let Ok(token) = self.peek() {
            match token {
                Token::VAL(s) => {
                    self.next();
                    vs.push(s.to_owned());
                },
                Token::BLT(s) |
                Token::KWD(s) => {
                    if param.choices.iter().any(|i| i == &s) {
                        self.next();
                        vs.push(s.to_owned());
                    }
                    else {
                        break;
                    }
                },
                Token::KVP(s) => {
                    return Err(ParseError::Custom("parse_many: not supported yet".to_string()))
                },
                _ => break,
            }
        }
        if vs.len() == 0 {
            return Err(ParseError::Custom(format!("parse_many: at_least_one={}, not found", at_least_one)))

        }
        param.value = Value::List(vs);
        Ok(())
    }
    pub fn parse_one(&mut self, param: &mut Param) -> Result<()> {
        let token = self.peek()?;
        println!("parse_one: token={}", token);
        match token {
            Token::VAL(s) => {
                self.next();
                param.value = Value::Item(s.to_owned());
            },
            Token::BLT(s) |
            Token::KWD(s) => {
                if param.choices.iter().any(|i| i == &s) {
                    self.next();
                    param.value = Value::Item(s.to_owned());
                }
                else {
                    return Err(ParseError::ProtectedNotChoice(Token::KWD(s.to_owned()), param.choices.to_owned()))
                }
            },
            Token::KVP(s) => {
                let parts: Vec<String> = s.split('=').map(|s| s.to_string()).collect();
                self.next();
                let mut dict = HashMap::new();
                dict.insert(parts[0].to_owned(), parts[1].to_owned());
                param.value = Value::Dict(dict);
            },
            Token::ARG(s) => {
                return Err(ParseError::UnexpectedToken(Token::ARG(s.to_owned())))
            }
            Token::REM(vs) => {
                return Err(ParseError::UnexpectedToken(Token::REM(vs.to_owned())))
            }
            Token::EOF => {
                return Err(ParseError::UnexpectedToken(Token::EOF))
            }
        }
        Ok(())
    }
    pub fn parse_zero(&mut self, param: &mut Param) -> Result<()> {
        println!("parse_zero: ");
        param.value = param.constant.to_owned();
        Ok(())
    }
    pub fn parse_one_or_zero(&mut self, param: &mut Param) -> Result<()> {
        println!("parse_one_or_zero: ");
        match self.parse_one(param) {
            Ok(()) => Ok(()),
            Err(e) => self.parse_zero(param),
        }
    }
    pub fn parse_one_or_more(&mut self, param: &mut Param) -> Result<()> {
        self.parse_many(param, true)?;
        Ok(())
    }
    pub fn parse_zero_or_more(&mut self, param: &mut Param) -> Result<()> {
        self.parse_many(param, false)?;
        Ok(())
    }
    pub fn parse_range(&mut self, param: &mut Param, min: usize, max: usize) -> Result<()> {
        Ok(())
    }
    pub fn peek(&mut self) -> Result<Token> {
        match self.tokens.get(self.index) {
            Some(token) => Ok(token.to_owned()),
            None => Err(ParseError::Custom(format!("peek: unexpected error; self.index={}", self.index))),
        }
    }
    pub fn next(&mut self) -> Result<Token> {
        let token = self.peek()?;
        self.index += 1;
        Ok(token)
    }
}
