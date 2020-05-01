use std::fs::File;
use std::io::Read;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

fn default_otto() -> String {
    "otto".to_string()
}

fn default_version() -> i32 {
    1
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Spec {
    otto: Otto,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Otto {
    #[serde(default = "default_version")]
    version: i32,

    #[serde(default = "default_otto")]
    name: String,

    params: Option<HashMap<String, Param>>,

    action: Option<String>,

    tasks: Option<HashMap<String, Task>>,
}

#[derive(Default, Debug, PartialEq, Serialize, Deserialize)]
struct Task {
    name: Option<String>,

    params: Option<HashMap<String, Param>>,

    action: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Param {
    name: Option<String>,

    help: Option<String>,
}

fn main() {
    let filename = "examples/ex1.yml";
    match File::open(filename) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();

            let spec: Spec = serde_yaml::from_str(&content).unwrap();
            println!("{:?}", spec);
        }
        Err(error) => {
            println!("There is an error {}: {}", filename, error);
        }
    }
}
