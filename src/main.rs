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
    println!("args: {:#?}", args);

    let verbose = true;
    if verbose {
        info!("args = {:?}", args);
        warn!("hi");
    }

    let filename = "examples/ex1.yml";
    let loader = Loader::new();
    //println!("loader: {:#?}", loader);

    let spec = loader.load(filename).unwrap();
    println!("spec: {:#?}", spec);

    let mut parser = Parser::new(spec.clone());
    //println!("parser: {:#?}", parser);

    let parsed = parser.parse(&args)?;
    println!("************************************************************************");
    println!("parsed: {:#?}", parsed);

    let otto = spec.otto.clone();
    /*
    let c = Token::SHT("-c".to_string());
    let filename_ = Token::VAL("examples/ex1.yml".to_string());
    let otto_ = Token::KWD("otto".to_string());
    let hello = Token::KWD("hello".to_string());
    let name = Token::LNG("--name".to_string());
    let scott = Token::VAL("scott".to_string());
    let pets = Token::LNG("--pets".to_string());
    let bill = Token::VAL("bill".to_string());
    let frank = Token::VAL("frank".to_string());

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
    */
    Ok(())
}
