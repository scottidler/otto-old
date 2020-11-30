use std::env;
use log::{info,warn};
use std::path::{Path, PathBuf};
use clap::{Arg, App, Subcommand};

mod loader;

#[derive(Debug, PartialEq)]
pub struct Otto {
    specfile: PathBuf,
}

impl Otto {
    pub fn new(specfile: &PathBuf) -> Self {
        Self {
            specfile: PathBuf::from(specfile),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let verbose = true;
    if verbose {
        info!("args = {:?}", args);
        warn!("hi");
    }
    let filename = "examples/ex1.yml";
    let spec = loader::load(filename).unwrap();

    let s1 = String::from("s1".to_string());
    let task_names_and_helps = spec.task_names_and_helps();
    let mut subcommands: Vec<App> = vec![];
    for (name, help) in task_names_and_helps {
        let app = App::new(name).about(&*help);
        subcommands.push(app);
    }
    /*
    let names_and_helps = spec.task_names_and_helps();
    let subcommands: Vec<App> = names_and_helps
        .clone()
        .into_iter()
        .map(|(n,h)| App::new(n).about(&*h))
        .collect();
    */
    println!("subcommands={:#?}", subcommands);

    let app = App::new("otto")
        .version("v0.0.1")
        .author("Scott A. Idler <scott.a.idler@gmail.com>")
        .about("yaml-based task runner (like make|doit)")
        .arg(Arg::new("config")
            .short('s')
            .long("specfile")
            .value_name("SPEC")
            .default_value("otto.yml")
            .about("specfile to drive otto"))
        .subcommands(subcommands);
    /*
    for name in spec.task_names() {
        app.subcommand(App::new(name).about("info about the subcommand"));
    }
    */
    let matches = app.get_matches();
}
