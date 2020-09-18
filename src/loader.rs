use std::fs;

#[allow(unused_imports)]
use clap::{App, Arg, Subcommand};

use std::fmt;
use std::vec::Vec;
use serde::Deserialize;
use serde::de::{Deserializer, Visitor, MapAccess};
use anyhow::{Context,Result};

fn default_otto() -> String {
    "otto".to_string()
}

fn default_version() -> i32 {
    1
}

fn default_jobs() -> i32 {
    12
}

#[derive(Debug, PartialEq, Deserialize)]
struct Spec {
    otto: Otto,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Otto {
    #[serde(default = "default_version")]
    version: i32,

    #[serde(default = "default_jobs")]
    jobs: i32,

    #[serde(default = "default_otto")]
    name: String,

    #[serde(default, deserialize_with = "de_param_map")]
    params: Vec<Param>,

    action: Option<String>,

    #[serde(deserialize_with = "de_task_map")]
    tasks: Vec<Task>,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Param {
    #[serde(skip_deserializing)]
    name: String,

    #[serde(default)]
    metavar: Option<String>,

    #[serde(default)]
    default: Option<String>,

    #[serde(default)]
    constant: Option<String>,

    #[serde(default)]
    choices: Option<String>, //FIXME: is this correct? probably not

    #[serde(default)]
    nargs: Option<String>,

    #[serde(default)]
    help: Option<String>,
}

#[derive(Default, Debug, PartialEq, Deserialize)]
struct Task {
    #[serde(skip_deserializing)]
    name: String,

    #[serde(default, deserialize_with = "de_param_map")]
    params: Vec<Param>,

    action: String,
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
                param.name = name;
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

pub fn load(filename: &str) -> Result<()> {
    let content = fs::read_to_string(filename).context(format!("Can't load filename={:?}", filename))?;
    let spec: Spec = serde_yaml::from_str(&content).context(format!("Can't load content={:?}", content))?;
    println!("{:#?}", spec);
    Ok(())
}
