// mod calculator;
// mod errors;
// mod forth;
// mod stack;

use std::env;

use rust_forth::{Config, forth::parser::Parser};

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let parser = Parser::new();
    let config = Config::build(&args, &parser);

    if let Ok(config) = config {
        if let Err(e) = rust_forth::run(config, &parser) {
            println!("Error to run program: {}", e);
        }
    }
}
