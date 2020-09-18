use std::env;

mod loader;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    println!("args = {:?}", args);
    let filename = "examples/ex1.yml";
    if let Err(err) = loader::load(filename) {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }
}
