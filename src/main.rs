use anyhow::Result;
use log::{info, warn};
use std::env;

pub mod cfg;
pub mod cli;

use cli::ast::AST;
use cli::token::Token;
use cli::parser::Parser;
use cfg::loader::Loader;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    //println!("args: {:#?}", args);

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
    //println!("loader: {:#?}", loader);

    let spec = loader.load(filename).unwrap();
    //println!("spec: {:#?}", spec);

    let parser = Parser::new(spec.clone());
    //println!("parser: {:#?}", parser);

    let parsed = parser.parse(args);
    println!("parsed: {:#?}", parsed);

    let otto = spec.otto.clone();

    let c = Token::SHT("-c".to_string());
    let filename_ = Token::VAL("examples/ex1.yml".to_string());
    let otto_ = Token::TSK("otto".to_string());
    let hello = Token::TSK("hello".to_string());
    let name = Token::LNG("--name".to_string());
    let scott = Token::VAL("scott".to_string());
    let pets = Token::LNG("--pets".to_string());
    let bill = Token::VAL("bill".to_string());
    let frank = Token::VAL("frank".to_string());
    let eof = Token::EOF;

    let cmd = AST::Cmd(otto_,
        vec![
            AST::Assign(c, Box::new(AST::Atom(filename_))),
            AST::Cmd(hello,
            vec![
                AST::Assign(name, Box::new(AST::Atom(scott))),
                AST::Assign(pets, Box::new(AST::Array(vec![bill, frank]))),
            ]),
        ]);
    println!("otto: {}", cmd);
    Ok(())

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
