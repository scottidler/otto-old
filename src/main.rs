use std::fs::File;
use std::io::Read;

#[allow(unused_imports)]
use clap::{App, Arg, Subcommand};

use std::fmt;
use std::vec::Vec;
use serde::Deserialize;
use serde::de::{Deserializer, Visitor, MapAccess};

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
    metavar: String,

    #[serde(default)]
    default: String,

    #[serde(default)]
    constant: String,

    #[serde(default)]
    choices: String, //FIXME: is this correct? probably not

    #[serde(default)]
    nargs: String,

    #[serde(default)]
    help: String,
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

fn main() {
    /*
    let matches = App::new("MyApp")
        // Normal App and Arg configuration goes here...
        // In the following example assume we wanted an application which
        // supported an "add" subcommand, this "add" subcommand also took
        // one positional argument of a file to add:
        .subcommand(
            App::new("add") // The name we call argument with
                .about("Adds files to myapp") // The message displayed in "myapp -h"
                // or "myapp help"
                .version("0.1") // Subcommands can have independent version
                .author("Kevin K.") // And authors
		.arg(
		    Arg::with_name("awesome")
			.about("turns up the awesome") // Displayed when showing help info
			.short('a') // Trigger this arg with "-a"
			.long("awesome") // Trigger this arg with "--awesome"
			.multiple(true) // This flag should allow multiple
			// occurrences such as "-aaa" or "-a -a"
		)
        )
        .subcommand(
            App::new("sub") // The name we call argument with
                .about("Subs files to myapp") // The message displayed in "myapp -h"
                // or "myapp help"
                .version("0.1") // Subcommands can have independent version
                .author("Kevin K.") // And authors
		.arg(
		    Arg::with_name("radical")
			.about("turns up the radical") // Displayed when showing help info
			.short('r') // Trigger this arg with "-a"
			.long("radical") // Trigger this arg with "--awesome"
			.multiple(true) // This flag should allow multiple
			// occurrences such as "-aaa" or "-a -a"
		)
        )
        .get_matches();

    println!("{:?}", matches);
    println!();
*/
    let filename = "examples/ex1.yml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let spec: Spec = serde_yaml::from_str(&content).unwrap();
            println!("{:#?}", spec);
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
        }
    }
}
