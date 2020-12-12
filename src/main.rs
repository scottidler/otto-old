use std::env;
use log::{info,warn};
/*
use std::path::{Path, PathBuf};
use clap::{Arg, App, Subcommand};
*/

pub mod cli;
pub mod cfg;

use cfg::loader::Loader;
use cli::lexer::Lexer;
use cli::parser::Parser;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args: {:#?}", args);

    let verbose = true;
    if verbose {
        info!("args = {:?}", args);
        warn!("hi");
    }

    /*
    let filename = match args.first() {
        Some(s) => s,
        None => "examples/ex1.yml",
    };
    */

    let filename = "examples/ex1.yml";
    let loader = Loader::new();
    println!("loader: {:#?}", loader);

    let otto = loader.load(filename).unwrap();
    println!("otto: {:#?}", otto);

    let lexer = Lexer::new();
    println!("lexer: {:#?}", lexer);

    let tokens = lexer.tokenize(args);
    println!("tokens: {:#?}", tokens);

    let parser = Parser::new(otto);
    println!("parser: {:#?}", parser);

    let parsed = parser.parse(tokens);
    println!("parsed: {:#?}", parsed);

    /*
    let subcommands: Vec<App> = otto.task_names_and_helps()
        .map(|(n,h)| App::new(n).about(h))
        .collect();
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
    let matches = app.get_matches();
    */
}
