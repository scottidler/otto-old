use std::fs;
use std::fmt;
use std::vec::Vec;
use serde::Deserialize;
use serde::de::{Deserializer, Visitor, MapAccess};
use anyhow::{Context,Result};

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

    #[serde(default, deserialize_with = "de_param_map")]
    pub params: Vec<Param>,

    pub action: Option<String>,

    #[serde(default, deserialize_with = "de_task_map")]
    pub tasks: Vec<Task>,
}

impl Otto {
    pub fn task_names(&self) -> Vec<&str> {
        return self.tasks
            .iter()
            .map(|t| t.name.as_str())
            .collect()
    }
    pub fn task_names_and_helps(&self) -> Vec<(&str, &str)> {
        self.tasks
            .iter()
            .map(|t| (t.name.as_str(), t.help.as_str()))
            .collect()
    }
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

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Param {
    #[serde(skip_deserializing)]
    pub name: String,

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

    #[serde(default, deserialize_with = "de_param_map")]
    pub params: Vec<Param>,

    #[serde(default)]
    pub after: Vec<String>,

    #[serde(default)]
    pub before: Vec<String>,

    pub action: Option<String>,
}

impl Task {
    pub fn param_names(&self) -> Vec<String> {
        self.params
            .clone()
            .into_iter()
            .map(|p| p.name)
            .collect()
    }
}

fn de_param_map<'de, D>(deserializer: D) -> Result<Vec<Param>, D::Error>
where
    D: Deserializer<'de>,
{
    struct ParamMap;

    impl<'de> Visitor<'de> for ParamMap {
        type Value = Vec<Param>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of name to Param")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut params = Vec::new();
            while let Some((name, mut param)) = map.next_entry::<String, Param>()? {
                // set the param.name as the parent of the param
                param.name = name;
                // set the default param.dest based upon the param.name
                if param.dest.is_none() {
                    let dest = param.name
                        .split('|')
                        .last()
                        .unwrap()
                        .trim_matches('-');
                    param.dest = Some(String::from(dest));
                }
                params.push(param);
            }
            Ok(params)
        }
    }

    deserializer.deserialize_map(ParamMap)
}

fn de_task_map<'de, D>(deserializer: D) -> Result<Vec<Task>, D::Error>
where
    D: Deserializer<'de>,
{
    struct TaskMap;

    impl<'de> Visitor<'de> for TaskMap {
        type Value = Vec<Task>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a map of name to Task")
        }

        fn visit_map<M>(self, mut map: M) -> Result<Self::Value, M::Error>
        where
            M: MapAccess<'de>,
        {
            let mut tasks = Vec::new();
            while let Some((name, mut task)) = map.next_entry::<String, Task>()? {
                task.name = name;
                tasks.push(task);
            }
            Ok(tasks)
        }
    }

    deserializer.deserialize_map(TaskMap)
}

pub fn load(filename: &str) -> Result<Otto, anyhow::Error> {
    let content = fs::read_to_string(filename).context(format!("Can't load filename={:?}", filename))?;
    let spec: Spec = serde_yaml::from_str(&content).context(format!("Can't load content={:?}", content))?;
    Ok(spec.otto)
}
