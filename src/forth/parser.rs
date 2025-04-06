use crate::errors::Error;

use super::intructions::DefineWord;
use super::word::{Word, WordManager};
use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
use crate::forth::intructions::ForthInstruction;
use crate::stack::stack_operations::StackOperation;

pub const LESS_THAN: LogicalOperation = LogicalOperation::LessThan;
pub const GREATER_THAN: LogicalOperation = LogicalOperation::GreaterThan;
pub const EQUAL: LogicalOperation = LogicalOperation::Equal;

pub const AND: BooleanOperation = BooleanOperation::And;
pub const OR: BooleanOperation = BooleanOperation::Or;
pub const NOT: BooleanOperation = BooleanOperation::Not;

pub const DUP: StackOperation = StackOperation::Dup;
pub const DROP: StackOperation = StackOperation::Drop;
pub const SWAP: StackOperation = StackOperation::Swap;
pub const OVER: StackOperation = StackOperation::Over;
pub const ROT: StackOperation = StackOperation::Rot;

#[derive(Debug, PartialEq)]
pub enum ParserState {
    OutsideDefinition,
    InsideDefinition,
    ParsingWordName,
}

/// Parser for Forth instructions
/// This struct is responsible for parsing Forth instructions from a string input.
#[derive(Debug, PartialEq)]
pub struct Parser {}

impl Default for Parser {
    fn default() -> Self {
        Self::new()
    }
}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    /// Parses a string input into a vector of Forth instructions.
    /// It tokenizes the input string and then parses each token to create the corresponding Forth instruction.
    /// Returns a vector of Forth instructions.
    /// # Arguments
    /// * `input` - A string containing the Forth instructions to be parsed.
    /// # Examples
    /// ```
    /// use rust_forth::forth::parser::Parser;
    /// use rust_forth::forth::intructions::ForthInstruction;
    /// use rust_forth::forth::word::WordManager;
    /// let parser = Parser::new();
    /// let word_manager = WordManager::new();
    /// let input = String::from("1 2 +");
    /// let expected_result = vec![
    ///     ForthInstruction::Number(1),
    ///     ForthInstruction::Number(2),
    ///     ForthInstruction::Operator("+".to_string()),
    /// ];
    /// let result = parser.parse_instructions(input, &word_manager);
    /// assert_eq!(result, expected_result);
    /// ```
    pub fn parse_instructions(&self, input: String, word_manager: &WordManager) -> Vec<ForthInstruction> {
        let mut instructions = Vec::new();
        let tokens = self.tokenize(&input);
        let mut state = ParserState::OutsideDefinition;

        for token in tokens {
            self.parse_token(token, &mut instructions, &mut state, word_manager);
        }

        instructions
    }

    /// Tokenizes the input string into a vector of tokens.
    /// It splits the input string by whitespace and special characters, handling quoted strings separately.
    /// Returns a vector of tokens.
    /// # Arguments
    /// * `input` - A string containing the input to be tokenized.
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut start = 0;
        let chars: Vec<char> = input.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '.' && input[i..].starts_with(".\" ") {
                if start < i {
                    tokens.push(input[start..i].to_string());
                }
                start = i;
                i += 2;
                in_quotes = true;

                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }

                if i < chars.len() && chars[i] == '"' {
                    tokens.push(input[start..=i].to_string());
                    i += 1;
                }
                if in_quotes {
                    in_quotes = false;
                }
                start = i;
            } else if chars[i].is_whitespace() && !in_quotes {
                if start < i {
                    tokens.push(input[start..i].to_string());
                }
                start = i + 1;
                i += 1;
            } else if !in_quotes && matches!(chars[i], ':' | ';') {
                // println!("i: {}", i);
                // println!("chars[i]: {}", chars[i]);
                if start < i {
                    tokens.push(input[start..i].to_string());
                }
                tokens.push(input[i..=i].to_string());
                start = i + 1;
                i += 1;
            } else {
                i += 1;
            }
        }
        if start < input.len() {
            tokens.push(input[start..].to_string());
        }
        tokens
    }

    /// Parses a token into a Forth instruction.
    /// It checks if the token is a number, operator, logical operation, boolean operation, stack operation,
    /// or a word. It then creates the corresponding Forth instruction and adds it to the instructions vector.
    /// # Arguments
    /// * `token` - A string containing the token to be parsed.
    /// * `instructions` - A mutable reference to a vector of Forth instructions where the parsed instruction will be added.
    fn parse_token(
        &self,
        token: String,
        instructions: &mut Vec<ForthInstruction>,
        state: &mut ParserState,
        word_manager: &WordManager,
    ) {
        match state {
            ParserState::OutsideDefinition => match token.as_str() {
                ":" => {
                    instructions.push(ForthInstruction::StartDefinition);
                    *state = ParserState::ParsingWordName;
                }
                ";" => instructions.push(ForthInstruction::EndDefinition),
                "." => instructions.push(ForthInstruction::OutputDot),
                _ if token.eq_ignore_ascii_case("emit") => {
                    instructions.push(ForthInstruction::OutpuEmit);
                    // println!("Error parsing token to emit: {:?}", token);
                }
                _ if token.eq_ignore_ascii_case("cr") => {
                    instructions.push(ForthInstruction::OutputCR)
                }
                _ if token.starts_with('.') && token.ends_with('"') => {
                    let quoted_string = &token[3..token.len() - 1];
                    instructions.push(ForthInstruction::OutputDotQuote(quoted_string.to_string()));
                }
                _ if self.is_number(token.to_string()) => {
                    if let Ok(parsed_num) = token.parse::<i16>() {
                        instructions.push(ForthInstruction::Number(parsed_num));
                    }
                }
                _ if self.is_operator(token.to_string()) => {
                    if word_manager.is_word_defined(&Word::UserDefined(token.to_string())) {
                        instructions.push(ForthInstruction::DefineWord(DefineWord::Name(
                            token.to_string().to_lowercase(),
                        )));
                    } else {
                        instructions.push(ForthInstruction::Operator(token));
                    }
                }
                _ if self.parse_stack_operation(&token, word_manager).is_some() => {
                    if let Some(stack_op) = self.parse_stack_operation(&token, word_manager) {
                        instructions.push(stack_op);
                    } else {
                        println!("Error parsing token to stack operation: {:?}", token);
                    }
                }
                _ if self.parse_logical_operation(&token).is_some() => {
                    if let Some(logical_op) = self.parse_logical_operation(&token) {
                        instructions.push(logical_op);
                    }
                }
                _ if self.parse_boolean_operation(&token).is_some() => {
                    if let Some(boolean_op) = self.parse_boolean_operation(&token) {
                        instructions.push(boolean_op);
                    }
                }
                _ if self.parse_word(&token, word_manager).is_some() => {
                    if let Some(word) = self.parse_word(&token, word_manager) {
                        instructions.push(word);
                    } else {
                        println!("Error parsing token to word: {:?}", token);
                    }
                }
                _ => {}
            },
            ParserState::ParsingWordName => {
                instructions.push(ForthInstruction::DefineWord(DefineWord::Name(token)));
                *state = ParserState::InsideDefinition;
            }
            ParserState::InsideDefinition => match token.as_str() {
                ";" => {
                    instructions.push(ForthInstruction::EndDefinition);
                    *state = ParserState::OutsideDefinition;
                }
                "." => instructions.push(ForthInstruction::OutputDot),
                _ if token.eq_ignore_ascii_case("emit") => {
                    instructions.push(ForthInstruction::OutpuEmit);
                    // println!("Error parsing token to emit: {:?}", token);
                }
                _ if token.eq_ignore_ascii_case("cr") => {
                    instructions.push(ForthInstruction::OutputCR)
                }
                _ if token.starts_with('.') && token.ends_with('"') => {
                    let quoted_string = &token[3..token.len() - 1];
                    instructions.push(ForthInstruction::OutputDotQuote(quoted_string.to_string()));
                }
                _ if self.is_number(token.to_string()) => {
                    if let Ok(parsed_num) = token.parse::<i16>() {
                        instructions.push(ForthInstruction::Number(parsed_num));
                    }
                }
                _ if self.is_operator(token.to_string()) => {
                    if word_manager.is_word_defined(&Word::UserDefined(token.to_string())) {
                        instructions.push(ForthInstruction::DefineWord(DefineWord::Name(
                            token.to_string(),
                        )));
                    } else {
                        instructions.push(ForthInstruction::Operator(token));
                    }
                }
                _ if self.parse_logical_operation(&token).is_some() => {
                    if let Some(logical_op) = self.parse_logical_operation(&token) {
                        instructions.push(logical_op);
                    }
                }
                _ if self.parse_boolean_operation(&token).is_some() => {
                    if let Some(boolean_op) = self.parse_boolean_operation(&token) {
                        instructions.push(boolean_op);
                    }
                }
                _ if self.parse_stack_operation(&token, word_manager).is_some() => {
                    if let Some(stack_op) = self.parse_stack_operation(&token, word_manager) {
                        instructions.push(stack_op);
                    } else {
                        println!("Error parsing token to stack operation: {:?}", token);
                    }
                }
                _ => {
                    if let Some(word) = self.parse_word(&token, word_manager) {
                        instructions.push(word);
                    } else {
                        println!("Error parsing token to word: {:?}", token);
                    }
                    // instructions.push(ForthInstruction::DefineWord(DefineWord::Name(token)));
                }
            },
            // "." => instructions.push(ForthInstruction::OutputDot),
            // _ if token.eq_ignore_ascii_case("emit") => {
            //     instructions.push(ForthInstruction::OutpuEmit);
            //     // println!("Error parsing token to emit: {:?}", token);
            // }
            // _ if token.eq_ignore_ascii_case("cr") => instructions.push(ForthInstruction::OutputCR),
            // _ if token.starts_with('.') && token.ends_with('"') => {
            //     let quoted_string = &token[3..token.len() - 1];
            //     instructions.push(ForthInstruction::OutputDotQuote(quoted_string.to_string()));
            // }
            // _ if self.parse_logical_operation(&token).is_some() => {
            //     if let Some(logical_op) = self.parse_logical_operation(&token) {
            //         instructions.push(logical_op);
            //     } else {
            //         println!("Error parsing token to logical operation: {:?}", token);
            //     }
            // }
            // _ if self.parse_boolean_operation(&token).is_some() => {
            //     if let Some(boolean_op) = self.parse_boolean_operation(&token) {
            //         instructions.push(boolean_op);
            //     } else {
            //         println!("Error parsing token to boolean operation: {:?}", token);
            //     }
            // }
            // _ if self.parse_word(&token, is_definition).is_some() => {
            //     if let Some(word) = self.parse_word(&token, is_definition) {
            //         instructions.push(word);
            //     } else {
            //         println!("Error parsing token to word: {:?}", token);
            //     }
            // }
            // _ if self.parse_stack_operation(&token).is_some() => {
            //     if let Some(stack_op) = self.parse_stack_operation(&token) {
            //         instructions.push(stack_op);
            //     } else {
            //         println!("Error parsing token to stack operation: {:?}", token);
            //     }
            // }
            // _ => (),
            // }
        }

        // match token.as_str() {
        //     num if self.is_number(num.to_string()) => {
        //         if let Ok(parsed_num) = num.parse::<i16>() {
        //             instructions.push(ForthInstruction::Number(parsed_num));
        //         }
        //     }
        //     op if self.is_operator(op.to_string()) => {
        //         instructions.push(ForthInstruction::Operator(op.to_string()))
        //     }
        //     ":" => instructions.push(ForthInstruction::StartDefinition),
        //     ";" => instructions.push(ForthInstruction::EndDefinition),
        //     "." => instructions.push(ForthInstruction::OutputDot),
        //     _ if token.eq_ignore_ascii_case("emit") => {
        //         instructions.push(ForthInstruction::OutpuEmit);
        //         // println!("Error parsing token to emit: {:?}", token);
        //     }
        //     _ if token.eq_ignore_ascii_case("cr") => instructions.push(ForthInstruction::OutputCR),
        //     _ if token.starts_with('.') && token.ends_with('"') => {
        //         let quoted_string = &token[3..token.len() - 1];
        //         instructions.push(ForthInstruction::OutputDotQuote(quoted_string.to_string()));
        //     }
        //     _ if self.parse_logical_operation(&token).is_some() => {
        //         if let Some(logical_op) = self.parse_logical_operation(&token) {
        //             instructions.push(logical_op);
        //         } else {
        //             println!("Error parsing token to logical operation: {:?}", token);
        //         }
        //     }
        //     _ if self.parse_boolean_operation(&token).is_some() => {
        //         if let Some(boolean_op) = self.parse_boolean_operation(&token) {
        //             instructions.push(boolean_op);
        //         } else {
        //             println!("Error parsing token to boolean operation: {:?}", token);
        //         }
        //     }
        //     _ if self.parse_word(&token, is_definition).is_some() => {
        //         if let Some(word) = self.parse_word(&token, is_definition) {
        //             instructions.push(word);
        //         } else {
        //             println!("Error parsing token to word: {:?}", token);
        //         }
        //     }
        //     _ if self.parse_stack_operation(&token).is_some() => {
        //         if let Some(stack_op) = self.parse_stack_operation(&token) {
        //             instructions.push(stack_op);
        //         } else {
        //             println!("Error parsing token to stack operation: {:?}", token);
        //         }
        //     }
        //     _ => (),
        // }
    }

    /// Checks if a token is a number.
    /// It checks if the token is a valid number, including negative numbers.
    /// # Arguments
    /// * `token` - A string containing the token to be checked.
    fn is_number(&self, token: String) -> bool {
        if token.is_empty() {
            return false;
        }
        let chars: Vec<char> = token.chars().collect();
        if chars[0] == '-' {
            return chars.len() > 1 && chars[1..].iter().all(|c| c.is_ascii_digit());
        }
        chars.iter().all(|c| c.is_ascii_digit())
    }

    /// Checks if a token is an operator.
    /// It matches the token against a set of known operators.
    /// Currently, it checks for the following operators: "+", "-", "*", "/".
    /// # Arguments
    /// * `token` - A string containing the token to be checked.
    fn is_operator(&self, token: String) -> bool {
        matches!(token.as_str(), "+" | "-" | "*" | "/")
    }

    /// Parses a token into a logical operation.
    /// It checks if the token is a logical operation and creates the corresponding Forth instruction.
    /// # Arguments
    /// * `token` - A string containing the token to be parsed.
    /// # Returns
    /// * `Some(ForthInstruction)` if the token is a logical operation.
    /// * `None` if the token is not a logical operation.
    fn parse_logical_operation(&self, token: &str) -> Option<ForthInstruction> {
        match token {
            _ if token.eq_ignore_ascii_case("<") => {
                Some(ForthInstruction::LogicalOperation(LESS_THAN))
            }
            _ if token.eq_ignore_ascii_case(">") => {
                Some(ForthInstruction::LogicalOperation(GREATER_THAN))
            }
            _ if token.eq_ignore_ascii_case("=") => Some(ForthInstruction::LogicalOperation(EQUAL)),
            _ => None,
        }
    }

    /// Parses a token into a boolean operation.
    /// It checks if the token is a boolean operation and creates the corresponding Forth instruction.
    /// # Arguments
    /// * `token` - A string containing the token to be parsed.
    /// # Returns
    /// * `Some(ForthInstruction)` if the token is a boolean operation.
    /// * `None` if the token is not a boolean operation.
    fn parse_boolean_operation(&self, token: &str) -> Option<ForthInstruction> {
        match token {
            _ if token.eq_ignore_ascii_case("and") => Some(ForthInstruction::BooleanOperation(AND)),
            _ if token.eq_ignore_ascii_case("or") => Some(ForthInstruction::BooleanOperation(OR)),
            _ if token.eq_ignore_ascii_case("not") => Some(ForthInstruction::BooleanOperation(NOT)),
            _ => None,
        }
    }

    /// Parses a token into a stack operation.
    /// It checks if the token is a stack operation and creates the corresponding Forth instruction.
    /// # Arguments
    /// * `token` - A string containing the token to be parsed.
    /// # Returns
    /// * `Some(ForthInstruction)` if the token is a stack operation.
    /// * `None` if the token is not a stack operation.
    fn parse_stack_operation(&self, token: &str, word_manager: &WordManager) -> Option<ForthInstruction> {
        if word_manager.is_word_defined(&Word::UserDefined(token.to_string())) {
            return Some(ForthInstruction::DefineWord(DefineWord::Name(
                token.to_string(),
            )));
        }
        
        match token {
            _ if token.eq_ignore_ascii_case("dup") => Some(ForthInstruction::StackWord(DUP)),
            _ if token.eq_ignore_ascii_case("drop") => Some(ForthInstruction::StackWord(DROP)),
            _ if token.eq_ignore_ascii_case("swap") => Some(ForthInstruction::StackWord(SWAP)),
            _ if token.eq_ignore_ascii_case("over") => Some(ForthInstruction::StackWord(OVER)),
            _ if token.eq_ignore_ascii_case("rot") => Some(ForthInstruction::StackWord(ROT)),
            _ => {
                println!("Unknown stack operation: {}", token);
                None
            }
        }
    }

    /// Parses a token into a word.
    /// It checks if the token is a word and creates the corresponding Forth instruction.
    /// # Arguments
    /// * `token` - A string containing the token to be parsed.
    /// # Returns
    /// * `Some(ForthInstruction)` if the token is a word.
    /// * `None` if the token is not a word.   
    fn parse_word(&self, token: &String, word_manager: &WordManager) -> Option<ForthInstruction> {
        if word_manager.is_word_defined(&Word::UserDefined(token.to_string())) {
            return Some(ForthInstruction::DefineWord(DefineWord::Name(
                token.to_string(),
            )));
        }

        match token.as_str() {
            _ if token.eq_ignore_ascii_case("if") => {
                Some(ForthInstruction::DefineWord(DefineWord::If))
            }
            _ if token.eq_ignore_ascii_case("else") => {
                Some(ForthInstruction::DefineWord(DefineWord::Else))
            }
            _ if token.eq_ignore_ascii_case("then") => {
                Some(ForthInstruction::DefineWord(DefineWord::Then))
            }
            // _ if self.parse_stack_operation(&token).is_some() => {
            //     self.parse_stack_operation(&token)
            // }
            _ => Some(ForthInstruction::DefineWord(DefineWord::Name(
                token.to_string().to_lowercase(),
            ))),
        }
    }

    /// Parses a stack size from a string input.
    /// It checks if the input string is in the format "stack-size=SIZE" and extracts the size.
    /// # Arguments
    /// * `input` - A string containing the stack size to be parsed.
    /// # Examples
    /// ```
    /// use rust_forth::forth::parser::Parser;
    /// use rust_forth::errors::Error;
    /// let parser = Parser::new();
    /// let input = "stack-size=1024";
    /// let expected_result: usize = 1024;
    /// let result = parser.parse_stack_size(input);
    /// assert_eq!(result, Ok(expected_result));
    /// ```
    /// # Returns
    /// * `Ok(usize)` if the input string is valid and the size is extracted.
    /// * `Err(Error)` if the input string is invalid or the size is not a valid number.
    pub fn parse_stack_size(&self, input: &str) -> Result<usize, Error> {
        let parts: Vec<&str> = input.split("=").collect();
        if parts.len() != 2 {
            return Err(Error::InvalidStackSize);
        }

        if let Ok(size) = parts[1].parse::<usize>() {
            return Ok(size);
        }
        Err(Error::InvalidStackSize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;
    use crate::{forth::intructions::ForthInstruction, stack::stack_operations::StackOperation};

    #[test]
    fn can_parse_simple_instructions() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("1 2 +");
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::Operator("+".to_string()),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_logical_instructions() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("1 2 <");
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_boolean_instructions() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("3 4 < 20 30 < AND");
        let expected_result = vec![
            ForthInstruction::Number(3),
            ForthInstruction::Number(4),
            ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
            ForthInstruction::Number(20),
            ForthInstruction::Number(30),
            ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
            ForthInstruction::BooleanOperation(BooleanOperation::And),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_intruction_that_manipulate_the_stack() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("1 2 3 DROP DUP SWAP");
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::Number(3),
            ForthInstruction::StackWord(StackOperation::Drop),
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::StackWord(StackOperation::Swap),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_defined_words() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("AWORD *WORD*");
        let expected_result = vec![
            ForthInstruction::DefineWord(DefineWord::Name("aword".to_string())),
            ForthInstruction::DefineWord(DefineWord::Name("*word*".to_string())),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definitions() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from(": NEGATE -1 * ;");
        let expected_result = vec![
            ForthInstruction::StartDefinition,
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())),
            ForthInstruction::Number(-1),
            ForthInstruction::Operator(String::from("*")),
            ForthInstruction::EndDefinition,
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_mixed_case_instructions() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("1 2 and Dup DroP");
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::BooleanOperation(BooleanOperation::And),
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::StackWord(StackOperation::Drop),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_ouput_generator_intruction() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from(". emit CR .\" Hello, World!\"");
        let expected_result = vec![
            ForthInstruction::OutputDot,
            ForthInstruction::OutpuEmit,
            ForthInstruction::OutputCR,
            ForthInstruction::OutputDotQuote(String::from("Hello, World!")),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definition_with_conditionals() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input =
            String::from(": is-negative? 0 < IF .\" Is negative\" ELSE .\" Is positive\" then ;");
        let expected_result = vec![
            ForthInstruction::StartDefinition,
            ForthInstruction::DefineWord(DefineWord::Name("is-negative?".to_string())),
            ForthInstruction::Number(0),
            ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
            ForthInstruction::DefineWord(DefineWord::If),
            ForthInstruction::OutputDotQuote(String::from("Is negative")),
            ForthInstruction::DefineWord(DefineWord::Else),
            ForthInstruction::OutputDotQuote(String::from("Is positive")),
            ForthInstruction::DefineWord(DefineWord::Then),
            ForthInstruction::EndDefinition,
        ];
        let result = parser.parse_instructions(input, &word_manager);
        dbg!(&result);
        dbg!(&expected_result);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_stack_size() {
        let parser = Parser::new();
        let input = "stack-size=1024";
        let expected_result: usize = 1024;

        let result = parser.parse_stack_size(input);

        assert_eq!(result, Ok(expected_result));
    }

    #[test]
    fn try_parse_invalid_stack_size_shoud_throw_error() {
        let parser = Parser::new();
        let input = "stack-size=1024a";

        let result = parser.parse_stack_size(input);

        assert!(result.is_err());
    }

    #[test]
    fn try_parse_negative_stack_size_shoud_throw_error() {
        let parser = Parser::new();
        let input = "stack-size=-1024";

        let result = parser.parse_stack_size(input);

        assert_eq!(result, Err(Error::InvalidStackSize));
    }

    #[test]
    fn test_case_insensitive_stack_operations() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("DUP drop swap OVER rOt");
        let expected_result = vec![
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::StackWord(StackOperation::Drop),
            ForthInstruction::StackWord(StackOperation::Swap),
            ForthInstruction::StackWord(StackOperation::Over),
            ForthInstruction::StackWord(StackOperation::Rot),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definition_with_reserved_words_correctly() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from(": dup-twice dup dup ;");
        let expected_result = vec![
            ForthInstruction::StartDefinition,
            ForthInstruction::DefineWord(DefineWord::Name("dup-twice".to_string())),
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::EndDefinition,
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_case_insensitive_words() {
        let parser = Parser::new();
        let word_manager = WordManager::new();
        let input = String::from("aWord Aword aword");
        let expected_result = vec![
            ForthInstruction::DefineWord(DefineWord::Name("aword".to_string())),
            ForthInstruction::DefineWord(DefineWord::Name("aword".to_string())),
            ForthInstruction::DefineWord(DefineWord::Name("aword".to_string())),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }
}
