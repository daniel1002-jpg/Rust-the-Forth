use rust_forth::{Config, forth::parser::Parser};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parser = Parser::new();
    let config = Config::build(&args, &parser);

    if let Ok(config) = config {
        if let Err(e) = rust_forth::run(config) {
            println!("{}", e);
            // println!("Error to run program: {}", e);
        }
    }
}
