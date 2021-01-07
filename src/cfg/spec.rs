use anyhow::{
    anyhow,
    Result,
};
use serde::de::{Deserializer, MapAccess, SeqAccess, Visitor, Error};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;
use std::num::ParseIntError;
/*
type Tasks = Vec<Task>;
type Params = Vec<Param>;
*/
type Tasks = HashMap<String, Task>;
type Params = HashMap<String, Param>;

#[derive(Debug, Clone, PartialEq)]
pub enum Nargs {
    One,
    Zero,
    OneOrZero,
    OneOrMore,
    ZeroOrMore,
    Range(usize, usize),
}

impl Default for Nargs {
    fn default() -> Self {
        Nargs::One
    }
}

impl<'de> Deserialize<'de> for Nargs {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        let result = match &s[..] {
            "1" => Nargs::One,
            "0" => Nargs::Zero,
            "?" => Nargs::OneOrZero,
            "+" => Nargs::OneOrMore,
            "*" => Nargs::ZeroOrMore,
            _ => {
                println!("s={}", s);
                if s.contains(":") {
                    let parts: Vec<&str> = s.split(":").collect();
                    let min = parts[0].parse::<usize>().unwrap(); //FIXME: this is awful
                    let max = parts[1].parse::<usize>().unwrap(); //FIXME: this is awful
                    Nargs::Range(min-1, max)
                } else {
                    let num = s.parse::<usize>().unwrap(); //FIXME: this is awful
                    Nargs::Range(0, num)
                }
            },
        };
        Ok(result)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Item(String),
    List(Vec<String>),
    Dict(HashMap<String,String>), //FIXME: no support for this yet
    Empty,
}

impl Default for Value {
    fn default() -> Self {
        Value::Empty
    }
}

fn deserialize_value<'de, D>(deserializer: D) -> Result<Value, D::Error>
    where D: Deserializer<'de>
{
    struct ValueEnum;
    impl<'de> Visitor<'de> for ValueEnum {
        type Value = Value;

        fn expecting(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
            fmtr.write_str("string or list of strings")
        }
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where E: Error
        {
            Ok(Value::Item(value.to_owned()))
        }
        fn visit_seq<S>(self, mut visitor: S) -> Result<Self::Value, S::Error>
            where S: SeqAccess<'de>
        {
            let mut vec: Vec<String> = vec![];
            while let Some(item) = visitor.next_element()? {
                vec.push(item);
            }
            Ok(Value::List(vec))
        }
    }
    deserializer.deserialize_any(ValueEnum)
}

fn default_otto() -> String {
    "otto".to_string()
}

fn default_verbosity() -> i32 {
    1
}

fn default_version() -> i32 {
    1
}

fn default_jobs() -> i32 {
    12
}
fn default_defaults() -> Defaults {
    Defaults {
        version: default_version(),
        verbosity: default_verbosity(),
        jobs: default_jobs(),
        tasks: vec![],
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Spec {
    #[serde(default = "default_defaults")]
    pub defaults: Defaults,

    pub otto: Otto,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_version")]
    pub version: i32,

    #[serde(default = "default_verbosity")]
    pub verbosity: i32,

    #[serde(default = "default_jobs")]
    pub jobs: i32,

    #[serde(default)]
    pub tasks: Vec<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Otto {

    #[serde(skip_deserializing, default = "default_otto")]
    pub name: String,

    #[serde(default)]
    pub help: Option<String>,

    #[serde(default, deserialize_with = "deserialize_param_map")]
    pub params: Params,

    #[serde(default, deserialize_with = "deserialize_task_map")]
    pub tasks: Tasks,

    pub action: Option<String>,
}

impl Otto {
    fn get_param_key(&self, flag: &String) ->Result<&String> {
        for (key, param) in self.params.iter() {
            if param.flags.iter().any(|f| f == flag) {
                return Ok(&key);
            }
        }
        Err(anyhow!("couldn't find key with flag given {}", flag))
    }
    pub fn get_param(&self, name: &String) -> Result<&Param> {
        self.params.get(name).ok_or(anyhow!("get_param: failed to get name={}", name))
    }
    pub fn get_param_from_flag(&self, flag: &String) -> Result<&Param> {
        let key = self.get_param_key(flag)?;
        self.get_param(key)
    }
    pub fn set_param(&mut self, param: Param) -> Result<Param> {
        let name = param.name.clone();
        self.params.insert(name.clone(), param).ok_or(anyhow!("set_param: failed to set param.name={}", name))
    }
    pub fn get_task(&self, name: &String) -> Result<&Task> {
        self.tasks.get(name).ok_or(anyhow!("get_task: failed to get param={}", name))
    }
    pub fn set_task(&mut self, task: Task) -> Result<Task> {
        let name = task.name.clone();
        self.tasks.insert(name.clone(), task).ok_or(anyhow!("set_task: failed to set task.name={}", name))
    }
}

pub enum ParamType {
    FLG,
    OPT,
    POS,
}

// FIXME: Flag, Named and Positional Args
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Param {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(skip_deserializing)]
    pub flags: Vec<String>,

    #[serde(skip_deserializing)]
    pub value: Value,

    #[serde(default)]
    pub dest: Option<String>,

    #[serde(default)]
    pub metavar: Option<String>,

    #[serde(default)]
    pub default: Option<String>,

    #[serde(default, deserialize_with = "deserialize_value")]
    pub constant: Value,

    #[serde(default)]
    pub choices: Vec<String>,

    #[serde(default)]
    pub nargs: Nargs,

    #[serde(default)]
    pub help: Option<String>,
}

impl Param {
    pub fn param_type(&self) -> ParamType {
        if self.flags.len() == 0 {
            return ParamType::POS;
        }
        ParamType::OPT
        /*
        match &self.nargs {
            Some(nargs) => {
                if nargs == "0" {
                    ParamType::FLG
                }
                else {
                    ParamType::OPT
                }
            }
            None => ParamType::OPT
        }
        */
    }
}

fn deserialize_param_map<'de, D>(deserializer: D) -> Result<Params, D::Error>
where
    D: Deserializer<'de>,
{
    struct ParamMap;

    impl<'de> Visitor<'de> for ParamMap {
        type Value = Params;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of name to Param")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut params = Params::new();
            while let Some((name, mut param)) = map.next_entry::<String, Param>()? {
                param.flags = name
                    .split('|')
                    .map(|f| f.to_string())
                    .collect();
                let short: Vec<String> = param.flags
                    .clone()
                    .into_iter()
                    .filter(|i| i.starts_with("-") && i.len() == 2)
                    .map(|i| i.to_string())
                    .collect();
                let long: Vec<String> = param.flags
                    .clone()
                    .into_iter()
                    .filter(|i| i.starts_with("--") && i.len() > 2)
                    .map(|i| i.to_string())
                    .collect();
                if param.dest.is_none() {
                    let dest = match long.first() {
                        Some(long) => String::from(long.trim_matches('-')),
                        None => match short.first() {
                            Some(short) => String::from(short.trim_matches('-')),
                            None => panic!("crash and burn"),
                        },
                    };
                    param.dest = Some(dest);
                }
                param.name = name.clone();
                params.insert(name.clone(), param);
            }
            Ok(params)
        }
    }
    deserializer.deserialize_map(ParamMap)
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Task {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(default)]
    pub help: Option<String>,

    #[serde(default)]
    pub after: Vec<String>,

    #[serde(default)]
    pub before: Vec<String>,

    #[serde(default, deserialize_with = "deserialize_param_map")]
    pub params: Params,

    pub action: Option<String>,

    #[serde(skip_deserializing)]
    pub selected: bool,
}

impl Task {
    fn get_param_key(&self, flag: &String) ->Result<&String> {
        for (key, param) in self.params.iter() {
            if param.flags.iter().any(|f| f == flag) {
                return Ok(&key);
            }
        }
        Err(anyhow!("couldn't find key with flag given {}", flag))
    }
    pub fn get_param(&self, name: &String) -> Result<&Param> {
        self.params.get(name).ok_or(anyhow!("get_param: failed to get name={}", name))
    }
    pub fn get_param_from_flag(&self, flag: &String) -> Result<&Param> {
        let key = self.get_param_key(flag)?;
        self.get_param(key)
    }
    pub fn set_param(&mut self, param: Param) -> Result<Param> {
        let name = param.name.clone();
        self.params.insert(name.clone(), param).ok_or(anyhow!("set_param: failed to set param.name={}", name))
    }
}

fn deserialize_task_map<'de, D>(deserializer: D) -> Result<Tasks, D::Error>
where
    D: Deserializer<'de>,
{
    struct TaskMap;

    impl<'de> Visitor<'de> for TaskMap {
        type Value = Tasks;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of name to Task")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut tasks = Tasks::new();
            while let Some((name, mut task)) = map.next_entry::<String, Task>()? {
                task.name = name.clone();
                tasks.insert(name.clone(), task);
            }
            Ok(tasks)
        }
    }
    deserializer.deserialize_map(TaskMap)
}
