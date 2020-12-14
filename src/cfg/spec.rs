use std::fmt;
use std::vec::Vec;
use std::collections::HashMap;
use serde::Deserialize;
use serde::de::{Deserializer, Visitor, MapAccess};
use anyhow::{Context,Result};

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

#[derive(Debug, PartialEq, Deserialize)]
pub struct Spec {
    pub otto: Otto,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Otto {

    #[serde(default = "default_otto")]
    pub name: String,

    #[serde(default = "default_version")]
    pub version: i32,

    pub defaults: Option<Defaults>,

    #[serde(default, deserialize_with = "deserialize_param_map")]
    pub params: Params,

    #[serde(default, deserialize_with = "deserialize_task_map")]
    pub tasks: Tasks,

    pub action: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Defaults {
    #[serde(default = "default_verbosity")]
    pub verbosity: i32,

    #[serde(default = "default_jobs")]
    pub jobs: i32,

    #[serde(default)]
    pub tasks: Vec<String>,
}

// FIXME: Flag, Named and Positional Args
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Param {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(skip_deserializing)]
    pub short: Vec<String>,

    #[serde(skip_deserializing)]
    pub long: Vec<String>,

    #[serde(skip_deserializing)]
    pub value: String,

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

#[derive(Default, Clone, Debug, PartialEq, Deserialize)]
pub struct Task {
    #[serde(skip_deserializing)]
    pub name: String,

    #[serde(default)]
    pub help: String,

    #[serde(default)]
    pub after: Vec<String>,

    #[serde(default)]
    pub before: Vec<String>,

    #[serde(default, deserialize_with = "deserialize_param_map")]
    pub params: Params,

    #[serde(default, deserialize_with = "deserialize_task_map")]
    pub tasks: Tasks,

    pub action: Option<String>,
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
                let flags: Vec<String> = name
                    .split('|')
                    .map(|i| i.to_string())
                    .collect();
                param.short = flags.clone()
                    .into_iter()
                    .filter(|i| i.starts_with("-") && i.len() == 2)
                    .map(|i| i.to_string())
                    .collect();
                param.long = flags.clone()
                    .into_iter()
                    .filter(|i| i.starts_with("--") && i.len() > 2)
                    .map(|i| i.to_string())
                    .collect();
                if param.dest.is_none() {
                    let dest = match param.long.first() {
                        Some(l) => String::from(l.trim_matches('-')),
                        None => {
                            match param.short.first() {
                                Some(s) => String::from(s.trim_matches('-')),
                                None => panic!("crash and burn")
                            }
                        }
                    };
                    param.dest = Some(dest);
                }
                println!("param.dest = {:#?}", param.dest);
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
