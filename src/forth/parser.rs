use crate::errors::Error;

use super::boolean_operations::{AND, EQUAL, GREATER_THAN, LESS_THAN, NOT, OR};
use super::definition_type::{DefinitionType, ELSE, IF, THEN};
use super::output_instructions::{CR, DOT, EMIT, OutputInstruction};
use super::word::{WordDefinitionManager, WordType};
use crate::forth::intruction::Instruction;
use crate::stack::stack_operations::{DROP, DUP, OVER, ROT, SWAP};

const START_DEFINITION: u8 = b':';
const END_DEFINITION: u8 = b';';

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
    ///# use rust_forth::forth::word::WordDefinitionManager;
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

    /// Normalizes a vector of tokens.
    /// Converts all tokens to lowercase except for quoted strings.
    ///
    /// # Returns
    /// A vector of normalized tokens.
    fn normalize_tokens(&self, tokens: Vec<String>) -> Vec<String> {
        tokens
            .into_iter()
            .map(|token| {
                if token.starts_with(".\"") && token.ends_with("\"") {
                    token
                } else {
                    token.to_lowercase()
                }
            })
            .collect()
    }

    /// Tries to process a quoted string in the input.
    ///
    /// # Arguments
    ///
    /// - `input` - A string containing the input to be processed.
    /// - `start` - The starting index to look for a quoted string.
    ///
    /// # Returns
    ///
    /// - `Some((String, usize))` if a quoted string is found, containing the quoted string and the new index.
    /// - `None` if no quoted string is found.
    fn try_process_quoted_string(&self, input: &str, start: usize) -> Option<(String, usize)> {
        if input[start..].starts_with(".\"") {
            let mut i = start + 2;
            while i < input.len() && input.as_bytes()[i] != b'"' {
                i += 1;
            }

            if i < input.len() && input.as_bytes()[i] == b'"' {
                let quoted_string = input[start..=i].to_string();
                return Some((quoted_string, i + 1));
            }
        }

        None
    }

    /// Tries to process a definition character in the input.
    /// A definition character is either START_DEFINITION or END_DEFINITION.
    ///
    /// # Arguments
    ///
    /// - `input` - A string containing the input to be processed.
    /// - `start` - The starting index to look for a definition character.
    ///
    /// # Returns
    ///
    /// - `Some((String, usize))` if a definition character is found, containing the character and the new index.
    /// - `None` if no definition character is found.
    fn try_process_definition_character(
        &self,
        input: &str,
        start: usize,
    ) -> Option<(String, usize)> {
        if matches!(input.as_bytes()[start], START_DEFINITION | END_DEFINITION) {
            let definition_char = input[start..=start].to_string();
            return Some((definition_char, start + 1));
        }

        None
    }

    /// Tries to process whitespace in the input.
    ///
    /// # Arguments
    ///
    /// - `input` - A string containing the input to be processed.
    /// - `start` - The starting index to look for whitespace.
    ///
    /// # Returns
    ///
    /// - `Some((String, usize))` if whitespace is found, containing an empty string and the new index.
    /// - `None` if no whitespace is found.
    fn try_process_whitespace(&self, input: &str, start: usize) -> Option<(String, usize)> {
        let mut i = start;
        while i < input.len() && input.as_bytes()[i].is_ascii_whitespace() {
            i += 1;
        }

        if start < i {
            return Some((String::new(), i));
        }

        None
    }

    /// Tries to process a generic token in the input.
    ///
    /// # Arguments
    ///
    /// - `input` - A string containing the input to be processed.
    /// - `start` - The starting index to look for a generic token.
    ///
    /// # Returns
    ///
    /// - `Some((String, usize))` if a generic token is found, containing the token and the new index.
    /// - `None` if no generic token is found.
    fn try_process_generic_token(&self, input: &str, start: usize) -> Option<(String, usize)> {
        let mut i = start;
        while i < input.len() && !self.is_especial_character(input.as_bytes()[i]) {
            i += 1;
        }

        if start < i {
            return Some((input[start..i].to_string(), i));
        }

        None
    }

    /// Checks if a character is an ASCII whitespace or a special character.
    /// A special character is defined as ':' or ';'.
    fn is_especial_character(&self, c: u8) -> bool {
        c.is_ascii_whitespace() || matches!(c, b':' | b';')
    }

    /// Tokenizes the input string into a vector of tokens.
    /// It splits the input string by whitespace and special characters, handling quoted strings separately.
    /// Returns a vector of tokens.
    ///
    /// # Arguments
    /// * `input` - A string containing the input to be tokenized.
    fn tokenize(&self, input: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut i = 0;

        while i < input.len() {
            if let Some((token, new_index)) = self
                .try_process_quoted_string(input, i)
                .or_else(|| self.try_process_definition_character(input, i))
                .or_else(|| self.try_process_whitespace(input, i))
                .or_else(|| self.try_process_generic_token(input, i))
            {
                if !token.is_empty() {
                    tokens.push(token);
                }
                i = new_index;
            } else {
                i += 1;
            }
        }

        self.normalize_tokens(tokens)
    }

    /// Processes a token and returns the corresponding Forth instruction.
    ///
    /// # Arguments
    /// * `token` - The token to be processed.
    /// * `word_manager` - The word_manager to check for defined words.
    ///
    /// # Returns
    /// - `Some(Instruction)` if the token is a valid instruction.
    /// - `None` if the token is not recognized.
    fn process_token(
        &self,
        token: &str,
        word_manager: &WordDefinitionManager,
    ) -> Option<Instruction> {
        self.parse_output_operation(token)
            .or_else(|| self.parse_number(token))
            .or_else(|| self.parse_operator(token, word_manager))
            .or_else(|| self.parse_logical_operation(token))
            .or_else(|| self.parse_boolean_operation(token))
            .or_else(|| self.parse_stack_operation(token, word_manager))
            .or_else(|| self.parse_word(token, word_manager))
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
            ParserState::OutsideDefinition => {
                if token == ":" {
                    instructions.push(Instruction::start_definition());
                    *state = ParserState::ParsingWordName;
                } else if token == ";" {
                    instructions.push(Instruction::end_definition());
                } else if let Some(instruction) = self.process_token(&token, word_manager) {
                    instructions.push(instruction);
                }
            }
            ParserState::ParsingWordName => {
                instructions.push(Instruction::definition_type(DefinitionType::name(token)));
                *state = ParserState::InsideDefinition;
            }
            ParserState::InsideDefinition => {
                if token == ";" {
                    instructions.push(Instruction::end_definition());
                    *state = ParserState::OutsideDefinition;
                } else if let Some(instruction) = self.process_token(&token, word_manager) {
                    instructions.push(instruction);
                }
            }
        }
    }

    /// Checks if a token is a number.
    /// It checks if the token is a valid number, including negative numbers.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be checked.
    fn is_number(&self, token: String) -> bool {
        token.parse::<i16>().is_ok()
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

    /// Parses a token into an output operation.
    /// It checks if the token is a recognized output operation and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a recognized output operation.
    /// - `None` if the token is not a recognized output operation.
    fn parse_output_operation(&self, token: &str) -> Option<Instruction> {
        match token {
            "." => Some(Instruction::output(DOT)),
            "emit" => Some(Instruction::output(EMIT)),
            "cr" => Some(Instruction::output(CR)),
            _ if token.starts_with(".\"") && token.ends_with("\"") => {
                let quoted_string = &token[3..token.len() - 1];
                Some(Instruction::output(OutputInstruction::dot_quote(
                    quoted_string.to_string(),
                )))
            }
            _ => None,
        }
    }

    /// Parses a token into a number.
    /// It checks if the token is a valid number and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a valid number.
    /// - `None` if the token is not a valid number.
    fn parse_number(&self, token: &str) -> Option<Instruction> {
        if self.is_number(token.to_string()) {
            token.parse::<i16>().ok().map(Instruction::number)
        } else {
            None
        }
    }

    /// Parses a token into an operator.
    /// It checks if the token is a recognized operator and creates the corresponding Forth instruction.
    ///
    /// # Arguments
    ///
    /// - `token` - A string containing the token to be parsed.
    ///
    /// # Returns
    ///
    /// - `Some(Instruction)` if the token is a recognized operator.
    /// - `None` if the token is not a recognized operator.
    fn parse_operator(
        &self,
        token: &str,
        word_manager: &WordDefinitionManager,
    ) -> Option<Instruction> {
        if self.is_operator(token.to_string()) {
            if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
                return Some(Instruction::definition_type(DefinitionType::Name(
                    token.to_lowercase(),
                )));
            }
            return Some(Instruction::operator(token.to_string()));
        }
        None
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
            "<" => Some(Instruction::logical_operation(LESS_THAN)),
            ">" => Some(Instruction::logical_operation(GREATER_THAN)),
            "=" => Some(Instruction::logical_operation(EQUAL)),
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
            "and" => Some(Instruction::boolean_operation(AND)),
            "or" => Some(Instruction::boolean_operation(OR)),
            "not" => Some(Instruction::boolean_operation(NOT)),
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
            "dup" => Some(Instruction::stack_word(DUP)),
            "drop" => Some(Instruction::stack_word(DROP)),
            "swap" => Some(Instruction::stack_word(SWAP)),
            "over" => Some(Instruction::stack_word(OVER)),
            "rot" => Some(Instruction::stack_word(ROT)),
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
    fn parse_word(&self, token: &str, word_manager: &WordDefinitionManager) -> Option<Instruction> {
        if word_manager.is_word_defined(&WordType::UserDefined(token.to_string())) {
            return Some(Instruction::definition_type(DefinitionType::name(
                token.to_string(),
            )));
        }

        Some(Instruction::definition_type(match token {
            "if" => IF,
            "else" => ELSE,
            "then" => THEN,
            _ => DefinitionType::name(token.to_string()),
        }))
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
    ///# use rust_forth::forth::parser::Parser;
    ///# use rust_forth::errors::Error;
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
            if size > 0 {
                return Ok(size);
            }
        }
        Err(Error::InvalidStackSize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::Error;
    use crate::forth::definition_type::THEN;
    use crate::forth::intruction::Instruction;

    #[test]
    fn can_parse_simple_instructions() {
        let parser = Parser::new();
        let word_manager = WordDefinitionManager::new();
        let input = String::from("1 2 +");
        let expected_result = vec![
            Instruction::number(1),
            Instruction::number(2),
            Instruction::operator("+".to_string()),
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
            Instruction::number(1),
            Instruction::number(2),
            Instruction::logical_operation(LESS_THAN),
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
            Instruction::number(3),
            Instruction::number(4),
            Instruction::logical_operation(LESS_THAN),
            Instruction::number(20),
            Instruction::number(30),
            Instruction::logical_operation(LESS_THAN),
            Instruction::boolean_operation(AND),
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
            Instruction::number(1),
            Instruction::number(2),
            Instruction::number(3),
            Instruction::stack_word(DROP),
            Instruction::stack_word(DUP),
            Instruction::stack_word(SWAP),
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
            Instruction::definition_type(DefinitionType::name("aword".to_string())),
            Instruction::definition_type(DefinitionType::name("*word*".to_string())),
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
            Instruction::start_definition(),
            Instruction::definition_type(DefinitionType::name("negate".to_string())),
            Instruction::number(-1),
            Instruction::operator(String::from("*")),
            Instruction::end_definition(),
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
            Instruction::number(1),
            Instruction::number(2),
            Instruction::boolean_operation(AND),
            Instruction::stack_word(DUP),
            Instruction::stack_word(DROP),
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
            Instruction::output(DOT),
            Instruction::output(EMIT),
            Instruction::output(CR),
            Instruction::output(OutputInstruction::dot_quote("Hello, World!".to_string())),
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
            Instruction::start_definition(),
            Instruction::definition_type(DefinitionType::name("is-negative?".to_string())),
            Instruction::number(0),
            Instruction::logical_operation(LESS_THAN),
            Instruction::definition_type(IF),
            Instruction::output(OutputInstruction::dot_quote("Is negative")),
            Instruction::definition_type(ELSE),
            Instruction::output(OutputInstruction::dot_quote("Is positive")),
            Instruction::definition_type(THEN),
            Instruction::end_definition(),
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
            Instruction::stack_word(DUP),
            Instruction::stack_word(DROP),
            Instruction::stack_word(SWAP),
            Instruction::stack_word(OVER),
            Instruction::stack_word(ROT),
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
            Instruction::start_definition(),
            Instruction::definition_type(DefinitionType::name("dup-twice".to_string())),
            Instruction::stack_word(DUP),
            Instruction::stack_word(DUP),
            Instruction::end_definition(),
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
            Instruction::definition_type(DefinitionType::name("aword".to_string())),
            Instruction::definition_type(DefinitionType::name("aword".to_string())),
            Instruction::definition_type(DefinitionType::name("aword".to_string())),
        ];

        let result = parser.parse_instructions(input, &word_manager);

        assert_eq!(result, expected_result);
    }
}
