use anyhow::{
    anyhow,
    Result,
};
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;
/*
type Tasks = Vec<Task>;
type Params = Vec<Param>;
*/
type Tasks = HashMap<String, Task>;
type Params = HashMap<String, Param>;

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

// FIXME: Flag, Named and Positional Args
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Param {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(skip_deserializing)]
    pub flags: Vec<String>,

    #[serde(skip_deserializing)]
    pub value: Option<String>,

    #[serde(default)]
    pub dest: Option<String>,

    #[serde(default)]
    pub metavar: Option<String>,

    #[serde(default)]
    pub default: Option<String>,

    #[serde(default)]
    pub constant: Option<String>,

    #[serde(default)]
    pub choices: Vec<String>,

    #[serde(default)]
    pub nargs: Option<String>,

    #[serde(default)]
    pub help: Option<String>,
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
    fn get_param_key(&self, flag: &String) -> Option<&String> {
        for (key, param) in self.params.iter() {
            if param.flags.iter().any(|f| f == flag) {
                Some(&key);
            }
        }
        None
    }
    pub fn get_param_from_flag(&self, flag: &String) -> Option<&Param> {
        let key = self.get_param_key(flag)?;
        self.params.get(key)
    }
    pub fn get_param(&self, name: &String) -> Option<&Param> {
        self.params.get(name)
    }
    pub fn set_param(&mut self, param: Param) -> Option<Param> {
        self.params.insert(param.name.clone(), param)
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
