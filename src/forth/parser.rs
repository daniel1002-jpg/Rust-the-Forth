use crate::errors::Error;

use super::definition_type::DefinitionType;
use super::word::{WordDefinitionManager, WordType};
use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
use crate::forth::intruction::Instruction;
use crate::stack::stack_operations::StackOperation;

/// Constants for logical operations
pub const LESS_THAN: LogicalOperation = LogicalOperation::LessThan;
pub const GREATER_THAN: LogicalOperation = LogicalOperation::GreaterThan;
pub const EQUAL: LogicalOperation = LogicalOperation::Equal;

/// Constants for boolean operations
pub const AND: BooleanOperation = BooleanOperation::And;
pub const OR: BooleanOperation = BooleanOperation::Or;
pub const NOT: BooleanOperation = BooleanOperation::Not;

/// Constants for stack operations
pub const DUP: StackOperation = StackOperation::Dup;
pub const DROP: StackOperation = StackOperation::Drop;
pub const SWAP: StackOperation = StackOperation::Swap;
pub const OVER: StackOperation = StackOperation::Over;
pub const ROT: StackOperation = StackOperation::Rot;

/// ParserState enum to represent the state of the parser
/// This enum is used to track whether the parser is currently inside a definition,
/// outside a definition, or parsing a word name.
///
/// # Variants
///
/// - `OutsideDefinition` - The parser is not currently inside a definition.
/// - `InsideDefinition` - The parser is currently inside a definition.
/// - `ParsingWordName` - The parser is currently parsing a word name.
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
    ///# use rust_forth::forth::parser::Parser;
    ///# use rust_forth::forth::intruction::Instruction;
    /// use rust_forth::forth::word::WordDefinitionManager;
    /// let parser = Parser::new();
    /// let word_manager = WordDefinitionManager::new();
    /// let input = String::from("1 2 +");
    /// let expected_result = vec![
    ///     Instruction::Number(1),
    ///     Instruction::Number(2),
    ///     Instruction::Operator("+".to_string()),
    /// ];
    /// let result = parser.parse_instructions(input, &word_manager);
    /// assert_eq!(result, expected_result);
    /// ```
    pub fn parse_instructions(
        &self,
        input: String,
        word_manager: &WordDefinitionManager,
    ) -> Vec<Instruction> {
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
    ///
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
                if start < i {
                    tokens.push(input[start..i].to_string());
                    println!("actual tokens: {:?}", tokens);
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
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    /// - `instructions` - A mutable reference to a vector of Forth instructions where the parsed instruction will be added.
    /// - `state` - A mutable reference to the current parser state.
    /// - `word_manager` - A reference to the WordDefinitionManager instance used to check if a word is defined.
    fn parse_token(
        &self,
        token: String,
        instructions: &mut Vec<Instruction>,
        state: &mut ParserState,
        word_manager: &WordDefinitionManager,
    ) {
        match state {
            ParserState::OutsideDefinition => match token.as_str() {
                ":" => {
                    instructions.push(Instruction::StartDefinition);
                    *state = ParserState::ParsingWordName;
                }
                ";" => instructions.push(Instruction::EndDefinition),
                "." => instructions.push(Instruction::OutputDot),
                _ if token.eq_ignore_ascii_case("emit") => {
                    instructions.push(Instruction::OutpuEmit);
                }
                _ if token.eq_ignore_ascii_case("cr") => instructions.push(Instruction::OutputCR),
                _ if token.starts_with('.') && token.ends_with('"') => {
                    let quoted_string = &token[3..token.len() - 1];
                    instructions.push(Instruction::OutputDotQuote(quoted_string.to_string()));
                }
                _ if self.is_number(token.to_string()) => {
                    if let Ok(parsed_num) = token.parse::<i16>() {
                        instructions.push(Instruction::Number(parsed_num));
                    }
                }
                _ if self.is_operator(token.to_string()) => {
                    if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
                        instructions.push(Instruction::DefinitionType(DefinitionType::Name(
                            token.to_string().to_lowercase(),
                        )));
                    } else {
                        instructions.push(Instruction::Operator(token));
                    }
                }
                _ if self.parse_stack_operation(&token, word_manager).is_some() => {
                    if let Some(stack_op) = self.parse_stack_operation(&token, word_manager) {
                        instructions.push(stack_op);
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
                    }
                }
                _ => {}
            },
            ParserState::ParsingWordName => {
                instructions.push(Instruction::DefinitionType(DefinitionType::Name(token)));
                *state = ParserState::InsideDefinition;
            }
            ParserState::InsideDefinition => match token.as_str() {
                ";" => {
                    instructions.push(Instruction::EndDefinition);
                    *state = ParserState::OutsideDefinition;
                }
                "." => instructions.push(Instruction::OutputDot),
                _ if token.eq_ignore_ascii_case("emit") => {
                    instructions.push(Instruction::OutpuEmit);
                }
                _ if token.eq_ignore_ascii_case("cr") => instructions.push(Instruction::OutputCR),
                _ if token.starts_with('.') && token.ends_with('"') => {
                    let quoted_string = &token[3..token.len() - 1];
                    instructions.push(Instruction::OutputDotQuote(quoted_string.to_string()));
                }
                _ if self.is_number(token.to_string()) => {
                    if let Ok(parsed_num) = token.parse::<i16>() {
                        instructions.push(Instruction::Number(parsed_num));
                    }
                }
                _ if self.is_operator(token.to_string()) => {
                    if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
                        instructions.push(Instruction::DefinitionType(DefinitionType::Name(
                            token.to_string(),
                        )));
                    } else {
                        instructions.push(Instruction::Operator(token));
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
                }
            },
        }
    }

    /// Checks if a token is a number.
    /// It checks if the token is a valid number, including negative numbers.
    /// # Arguments
    /// - `token` - A string containing the token to be checked.
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
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be checked.
    fn is_operator(&self, token: String) -> bool {
        matches!(token.as_str(), "+" | "-" | "*" | "/")
    }

    /// Parses a token into a logical operation.
    /// It checks if the token is a logical operation and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a logical operation.
    /// - `None` if the token is not a logical operation.
    fn parse_logical_operation(&self, token: &str) -> Option<Instruction> {
        match token {
            _ if token.eq_ignore_ascii_case("<") => Some(Instruction::LogicalOperation(LESS_THAN)),
            _ if token.eq_ignore_ascii_case(">") => {
                Some(Instruction::LogicalOperation(GREATER_THAN))
            }
            _ if token.eq_ignore_ascii_case("=") => Some(Instruction::LogicalOperation(EQUAL)),
            _ => None,
        }
    }

    /// Parses a token into a boolean operation.
    /// It checks if the token is a boolean operation and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    /// - `Some(Instruction)` if the token is a boolean operation.
    /// - `None` if the token is not a boolean operation.
    fn parse_boolean_operation(&self, token: &str) -> Option<Instruction> {
        match token {
            _ if token.eq_ignore_ascii_case("and") => Some(Instruction::BooleanOperation(AND)),
            _ if token.eq_ignore_ascii_case("or") => Some(Instruction::BooleanOperation(OR)),
            _ if token.eq_ignore_ascii_case("not") => Some(Instruction::BooleanOperation(NOT)),
            _ => None,
        }
    }

    /// Parses a token into a stack operation.
    /// It checks if the token is a stack operation and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a stack operation.
    /// - `None` if the token is not a stack operation.
    fn parse_stack_operation(
        &self,
        token: &str,
        word_manager: &WordDefinitionManager,
    ) -> Option<Instruction> {
        if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
            return Some(Instruction::DefinitionType(DefinitionType::Name(
                token.to_string(),
            )));
        }

        match token {
            _ if token.eq_ignore_ascii_case("dup") => Some(Instruction::StackWord(DUP)),
            _ if token.eq_ignore_ascii_case("drop") => Some(Instruction::StackWord(DROP)),
            _ if token.eq_ignore_ascii_case("swap") => Some(Instruction::StackWord(SWAP)),
            _ if token.eq_ignore_ascii_case("over") => Some(Instruction::StackWord(OVER)),
            _ if token.eq_ignore_ascii_case("rot") => Some(Instruction::StackWord(ROT)),
            _ => None,
        }
    }

    /// Parses a token into a word.
    /// It checks if the token is a word and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a word.
    /// - `None` if the token is not a word.   
    fn parse_word(
        &self,
        token: &String,
        word_manager: &WordDefinitionManager,
    ) -> Option<Instruction> {
        if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
            return Some(Instruction::DefinitionType(DefinitionType::Name(
                token.to_string(),
            )));
        }

        match token.as_str() {
            _ if token.eq_ignore_ascii_case("if") => {
                Some(Instruction::DefinitionType(DefinitionType::If))
            }
            _ if token.eq_ignore_ascii_case("else") => {
                Some(Instruction::DefinitionType(DefinitionType::Else))
            }
            _ if token.eq_ignore_ascii_case("then") => {
                Some(Instruction::DefinitionType(DefinitionType::Then))
            }
            _ => Some(Instruction::DefinitionType(DefinitionType::Name(
                token.to_string().to_lowercase(),
            ))),
        }
    }

    /// Parses a stack size from a string input.
    /// It checks if the input string is in the format "stack-size=SIZE" and extracts the size.
    ///
    /// # Arguments
    ///
    /// - `input` - A string containing the stack size to be parsed.
    ///
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
    ///
    /// - `Ok(usize)` if the input string is valid and the size is extracted.
    /// - `Err(Error)` if the input string is invalid or the size is not a valid number.
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
    use crate::{forth::intruction::Instruction, stack::stack_operations::StackOperation};

    #[test]
    fn can_parse_simple_instructions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("1 2 +");
        let expected_result = vec![
            Instruction::Number(1),
            Instruction::Number(2),
            Instruction::Operator("+".to_string()),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_logical_instructions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("1 2 <");
        let expected_result = vec![
            Instruction::Number(1),
            Instruction::Number(2),
            Instruction::LogicalOperation(LogicalOperation::LessThan),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_boolean_instructions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("3 4 < 20 30 < AND");
        let expected_result = vec![
            Instruction::Number(3),
            Instruction::Number(4),
            Instruction::LogicalOperation(LogicalOperation::LessThan),
            Instruction::Number(20),
            Instruction::Number(30),
            Instruction::LogicalOperation(LogicalOperation::LessThan),
            Instruction::BooleanOperation(BooleanOperation::And),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_intruction_that_manipulate_the_stack() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("1 2 3 DROP DUP SWAP");
        let expected_result = vec![
            Instruction::Number(1),
            Instruction::Number(2),
            Instruction::Number(3),
            Instruction::StackWord(StackOperation::Drop),
            Instruction::StackWord(StackOperation::Dup),
            Instruction::StackWord(StackOperation::Swap),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_defined_words() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("AWORD *WORD*");
        let expected_result = vec![
            Instruction::DefinitionType(DefinitionType::Name("aword".to_string())),
            Instruction::DefinitionType(DefinitionType::Name("*word*".to_string())),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definitions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from(": NEGATE -1 * ;");
        let expected_result = vec![
            Instruction::StartDefinition,
            Instruction::DefinitionType(DefinitionType::Name("NEGATE".to_string())),
            Instruction::Number(-1),
            Instruction::Operator(String::from("*")),
            Instruction::EndDefinition,
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_mixed_case_instructions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("1 2 and Dup DroP");
        let expected_result = vec![
            Instruction::Number(1),
            Instruction::Number(2),
            Instruction::BooleanOperation(BooleanOperation::And),
            Instruction::StackWord(StackOperation::Dup),
            Instruction::StackWord(StackOperation::Drop),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_ouput_generator_intruction() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from(". emit CR .\" Hello, World!\"");
        let expected_result = vec![
            Instruction::OutputDot,
            Instruction::OutpuEmit,
            Instruction::OutputCR,
            Instruction::OutputDotQuote(String::from("Hello, World!")),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definition_with_conditionals() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input =
            String::from(": is-negative? 0 < IF .\" Is negative\" ELSE .\" Is positive\" then ;");
        let expected_result = vec![
            Instruction::StartDefinition,
            Instruction::DefinitionType(DefinitionType::Name("is-negative?".to_string())),
            Instruction::Number(0),
            Instruction::LogicalOperation(LogicalOperation::LessThan),
            Instruction::DefinitionType(DefinitionType::If),
            Instruction::OutputDotQuote(String::from("Is negative")),
            Instruction::DefinitionType(DefinitionType::Else),
            Instruction::OutputDotQuote(String::from("Is positive")),
            Instruction::DefinitionType(DefinitionType::Then),
            Instruction::EndDefinition,
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
        let word_manager = WordDefinitionManager::new();
        let input = String::from("DUP drop swap OVER rOt");
        let expected_result = vec![
            Instruction::StackWord(StackOperation::Dup),
            Instruction::StackWord(StackOperation::Drop),
            Instruction::StackWord(StackOperation::Swap),
            Instruction::StackWord(StackOperation::Over),
            Instruction::StackWord(StackOperation::Rot),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definition_with_reserved_words_correctly() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from(": dup-twice dup dup ;");
        let expected_result = vec![
            Instruction::StartDefinition,
            Instruction::DefinitionType(DefinitionType::Name("dup-twice".to_string())),
            Instruction::StackWord(StackOperation::Dup),
            Instruction::StackWord(StackOperation::Dup),
            Instruction::EndDefinition,
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_case_insensitive_words() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("aWord Aword aword");
        let expected_result = vec![
            Instruction::DefinitionType(DefinitionType::Name("aword".to_string())),
            Instruction::DefinitionType(DefinitionType::Name("aword".to_string())),
            Instruction::DefinitionType(DefinitionType::Name("aword".to_string())),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }
}
