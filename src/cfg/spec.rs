use anyhow::{Context, Result};
use serde::de::{Deserializer, MapAccess, Visitor};
use serde::Deserialize;
use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;
/*
type Tasks = HashMap<String, Task>;
type Params = HashMap<String, Param>;
*/
type Tasks = Vec<Task>;
type Params = Vec<Param>;

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

#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Spec {
    pub defaults: Option<Defaults>,

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
    fn get_task_idx(&self, name: &String) -> Option<usize> {
        for (idx, task) in self.tasks.iter().enumerate() {
            if &task.name == name {
                return Some(idx);
            }
        }
        None
    }
    fn get_task(&self, name: &String) -> Option<&Task> {
        let idx = self.get_task_idx(name)?;
        self.tasks.get(idx)
    }
    fn get_param_idx(&self, flag: &String) -> Option<usize> {
        for (idx, param) in self.params.iter().enumerate() {
            if param.flags.iter().any(|f| f == flag) {
                return Some(idx);
            }
        }
        None
    }
    fn get_param(&self, flag: &String) -> Option<&Param> {
        let idx = self.get_param_idx(flag)?;
        self.params.get(idx)
    }
}

// FIXME: Flag, Named and Positional Args
#[derive(Clone, Debug, Default, PartialEq, Deserialize)]
pub struct Param {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(skip_deserializing)]
    pub flags: Vec<String>,

    /*
    #[serde(skip_deserializing)]
    pub short: Vec<String>,

    #[serde(skip_deserializing)]
    pub long: Vec<String>,
    */

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
                params.push(param);
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
                //tasks.insert(name.clone(), task);
                tasks.push(task);
            }
            Ok(tasks)
        }
    }
    deserializer.deserialize_map(TaskMap)
}
