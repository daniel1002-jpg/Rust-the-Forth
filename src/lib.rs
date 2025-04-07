pub mod calculator;
pub mod errors;
pub mod forth;
pub mod stack;

pub use forth::boolean_operations::{BooleanOperation, LogicalOperation};
pub use forth::interpreter::Forth;
pub use forth::intructions::ForthInstruction;
use forth::parser::Parser;
pub use stack::core::Stack;

use crate::errors::Error;
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub file_path: String,
    pub stack_size: Option<usize>,
}

impl Config {
    pub fn build(args: &[String], parser: &Parser) -> Result<Config, Error> {
        if args.len() < 2 || args[1].is_empty() {
            return Err(Error::MissingPathError);
        }

        let mut stack_size = None;
        if args.len() == 3 && !args[2].is_empty() {
            if let Ok(size) = parser.parse_stack_size(&args[2]) {
                stack_size = Some(size);
            } else {
                println!("invalid stack size");
                println!("using default stack size");
            }
        }

        let file_path = args[1].to_string();

        Ok(Config {
            file_path,
            stack_size,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open(&config.file_path)?;
    let reader = io::BufReader::new(file);
    let writer = io::BufWriter::new(io::stdout());
    let mut forth = Forth::new(config.stack_size, Some(writer));
    let stack_output = File::create("stack.fth")?;
    let mut stack_writer = io::BufWriter::new(stack_output);

    for line_result in reader.lines() {
        let line_readed = match line_result {
            Ok(line) => line,
            _ => "Error reading line".to_string(),
        };
        let tokens = forth.parse_instructions(line_readed.to_lowercase());
        // println!("\n");
        // dbg!("tokens: {:?}", &tokens);
        let instructions = tokens;
        forth.process_data(instructions)?;

        write_stack_output(&forth, &mut stack_writer)?;

        // println!("Line: {:?} success processed", line_readed.clone());
    }

    fn write_stack_output<W: Write>(
        forth: &Forth<W>,
        stack_writer: &mut BufWriter<File>,
    ) -> Result<(), io::Error> {
        let stack_content = forth.get_stack_content();
        let formatted_stack = stack_content
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ");
        writeln!(stack_writer, "{}", formatted_stack)?;
        Ok(())
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_stack_size_recibed_correctly() {
        let args = vec![
            "program_name".to_string(),
            "path_to_file.fth".to_string(),
            "stack-size=10".to_string(),
        ];
        let expected_path = "path_to_file.fth";
        let expected_size = Some(10);

        let parser = Parser::new();
        let config = Config::build(&args, &parser).unwrap();

        assert_eq!(config.file_path, expected_path);
        assert_eq!(config.stack_size, expected_size);
    }

    #[test]
    fn try_build_config_without_path_throw_error() {
        let args = vec!["program_name".to_string()];

        let parser = Parser::new();
        let config = Config::build(&args, &parser);

        assert_eq!(config, Err(Error::MissingPathError));
    }

    #[test]
    fn try_buid_config_with_empty_path_throw_error() {
        let args = vec![
            "program_name".to_string(),
            "".to_string(),
            "stack-size=1024".to_string(),
        ];

        let parser = Parser::new();
        let config = Config::build(&args, &parser);

        assert_eq!(config, Err(Error::MissingPathError));
    }
}
