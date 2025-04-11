use std::collections::HashMap;
use std::io::Write;
use std::vec;

use crate::errors::Error;
use crate::forth::boolean_operations::{FORTH_FALSE, FORTH_TRUE};
use crate::forth::definition_type::DefinitionType;
use crate::forth::forth_errors::ForthError;
use crate::forth::intruction::Instruction;
use crate::forth::word_data::WordData;
use crate::handler::instructions_handler::ExecutionHandler;
use crate::stack::stack_operations::StackOperation;
use crate::{BooleanOperation, LogicalOperation};

/// Constants that represents conditional words in Forth
const CONDITIONAL_IF: WordData = WordData::DefinitionType(DefinitionType::If);
const CONDITIONAL_THEN: WordData = WordData::DefinitionType(DefinitionType::Then);
const CONDITIONAL_ELSE: WordData = WordData::DefinitionType(DefinitionType::Else);

/// Enum that represents a word in the Forth language.
/// It can be either a predefined word (like "DUP") or a user-defined word (like "MY_WORD").
/// The `Word` enum is used to identify the type of word being defined or executed.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum WordType {
    Predefined(&'static str),
    UserDefined(String),
}

/// Struct that represents a word manager in the Forth interpreter
///
/// The `WordDefinitionManager` is responsible for managing the definitions of words in the Forth language.
/// It stores the definitions of words, their execution stack, and the current nesting level.
/// It also provides methods for defining new words, executing words, and checking if a word is defined.
pub struct WordDefinitionManager {
    words: HashMap<WordType, usize>,
    definitions: Vec<Vec<WordData>>,
    execution_stack: Vec<WordType>,
    nesting_level: usize,
}

impl Default for WordDefinitionManager {
    fn default() -> Self {
        WordDefinitionManager::new()
    }
}

impl WordDefinitionManager {
    /// Creates a new instance of the `WordDefinitionManager`.
    pub fn new() -> Self {
        WordDefinitionManager {
            words: HashMap::new(),
            definitions: Vec::new(),
            execution_stack: Vec::new(),
            nesting_level: 0,
        }
    }

    /// Defines a new word in the Forth interpreter.
    /// The word is defined by a name and a body of instructions.
    ///
    /// # Arguments
    ///
    /// - `name` - The name of the word to be defined.
    /// - `body` - The body of instructions that make up the word definition.
    ///
    /// # Example
    ///
    /// ```rust
    ///# use rust_forth::forth::word::{WordDefinitionManager, WordType};
    ///# use rust_forth::forth::intruction::Instruction;
    ///# use rust_forth::forth::word_data::WordData;
    ///
    /// let mut word_manager = WordDefinitionManager::new();
    /// let word_body = vec![
    ///     Instruction::Number(5),
    ///     Instruction::Operator("+".to_string()),
    ///     Instruction::EndDefinition,
    /// ];
    /// let result = word_manager.define_new_word(WordType::UserDefined("ADD-5".to_string()), word_body);
    ///
    /// assert!(word_manager.is_word_defined(&WordType::UserDefined("ADD-5".to_string())));
    ///
    /// let definition = word_manager.get_word_definition(&WordType::UserDefined("ADD-5".to_string()));
    /// assert!(definition.is_some());
    /// assert_eq!(definition.unwrap(), &vec![WordData::Number(5), WordData::Operator("+".to_string())]);
    /// ```
    pub fn define_new_word(&mut self, name: WordType, body: Vec<Instruction>) -> Result<(), Error> {
        if let WordType::UserDefined(ref name_str) = name {
            if !self.is_word_name_valid(name_str) {
                return Err(ForthError::InvalidWord.into());
            }
        }

        let end_index = find_end_definition(&body).ok_or(ForthError::InvalidWord)?;
        let word_definition = body.into_iter().take(end_index).collect::<Vec<_>>();
        let mut definition: Vec<WordData> = Vec::new();

        for element in word_definition {
            definition.extend(self.convert_to_word_definition(element)?);
        }

        let index = self.definitions.len();
        self.definitions.push(definition);
        self.words.insert(name, index);
        Ok(())
    }

    /// Converts a Forth instruction into a word definition.
    /// This function is used to expand the definition of a word into its individual components.
    fn convert_to_word_definition(
        &mut self,
        instruction: Instruction,
    ) -> Result<Vec<WordData>, Error> {
        match instruction {
            Instruction::Number(number) => self.convert_number(number),
            Instruction::Operator(operator) => self.convert_operator(operator),
            Instruction::StackWord(stack_word) => self.convert_stack_word(stack_word),
            Instruction::DefinitionType(define_word) => self.convert_define_word(define_word),
            Instruction::BooleanOperation(bool_op) => self.convert_boolean_operation(bool_op),
            Instruction::LogicalOperation(log_op) => self.convert_logical_operation(log_op),
            Instruction::OutputDot
            | Instruction::OutpuEmit
            | Instruction::OutputCR
            | Instruction::OutputDotQuote(_) => self.convert_output_instruction(instruction),
            _ => Ok(vec![]),
        }
    }

    fn convert_number(&self, number: i16) -> Result<Vec<WordData>, Error> {
        Ok(vec![WordData::Number(number)])
    }

    fn convert_operator(&self, operator: String) -> Result<Vec<WordData>, Error> {
        Ok(vec![WordData::Operator(operator)])
    }

    fn convert_stack_word(&self, stack_word: StackOperation) -> Result<Vec<WordData>, Error> {
        Ok(vec![WordData::StackWord(stack_word)])
    }

    fn convert_define_word(&self, define_word: DefinitionType) -> Result<Vec<WordData>, Error> {
        let mut definition = Vec::new();
        match define_word {
            DefinitionType::Name(name) => {
                if self.is_word_defined(&WordType::UserDefined(name.to_string())) {
                    if let Some(&index) = self.words.get(&WordType::UserDefined(name.to_string())) {
                        definition.push(WordData::DefinitionIndex(index));
                    }
                }
            }
            DefinitionType::If => definition.push(CONDITIONAL_IF),
            DefinitionType::Then => definition.push(CONDITIONAL_THEN),
            DefinitionType::Else => definition.push(CONDITIONAL_ELSE),
        }
        Ok(definition)
    }

    fn convert_boolean_operation(&self, bool_op: BooleanOperation) -> Result<Vec<WordData>, Error> {
        Ok(vec![WordData::BooleanOperation(bool_op)])
    }

    fn convert_logical_operation(&self, log_op: LogicalOperation) -> Result<Vec<WordData>, Error> {
        Ok(vec![WordData::LogicalOperation(log_op)])
    }

    fn convert_output_instruction(&self, instruction: Instruction) -> Result<Vec<WordData>, Error> {
        match instruction {
            Instruction::OutputDot => Ok(vec![WordData::OutputDot]),
            Instruction::OutpuEmit => Ok(vec![WordData::OutpuEmit]),
            Instruction::OutputCR => Ok(vec![WordData::OutputCR]),
            Instruction::OutputDotQuote(string) => {
                Ok(vec![WordData::OutputDotQuote(string.to_string())])
            }
            _ => Ok(vec![]),
        }
    }

    /// Executes a word in the Forth interpreter.
    /// This function takes a word name and executes the corresponding word definition.
    ///
    /// # Arguments
    ///
    /// - `handler` - The instruction handler that manages the execution of instructions.
    /// - `word_name` - The name of the word to be executed.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use rust_forth::forth::word::WordDefinitionManager;
    /// # use rust_forth::forth::intruction::Instruction;
    /// # use rust_forth::handler::instructions_handler::ExecutionHandler;
    /// # use rust_forth::forth::word::WordType;
    /// # use std::io::Sink;
    ///
    /// let mut word_manager = WordDefinitionManager::new();
    /// let word_body = vec![
    ///    Instruction::Number(5),
    ///    Instruction::Operator("+".to_string()),
    ///    Instruction::EndDefinition,
    /// ];
    /// let _ = word_manager.define_new_word(WordType::UserDefined("ADD-5".to_string()), word_body);
    /// let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
    ///
    /// let _ = handler.handle_push_element(10);
    /// let _ = word_manager.run_word(&mut handler, "ADD-5");
    ///
    /// assert_eq!(handler.handle_get_top_element(), Ok(&15));
    /// ```
    pub fn run_word<W: Write>(
        &mut self,
        handler: &mut ExecutionHandler<W>,
        word_name: &str,
    ) -> Result<(), Error> {
        self.execution_stack
            .push(WordType::UserDefined(word_name.to_string()));

        while let Some(current_word) = self.execution_stack.pop() {
            let index = self
                .words
                .get(&current_word)
                .ok_or(ForthError::UnknownWord)?;

            self.execute_instruction(handler, *index, 0)?;
        }

        self.execution_stack.clear();
        Ok(())
    }

    fn find_instruction_index(
        &self,
        def_index: usize,
        start: usize,
        target: WordData,
    ) -> Option<usize> {
        let instructions = self
            .definitions
            .get(def_index)
            .and_then(|def| def.get(start..))
            .unwrap_or(&[]);

        let mut nesting_level = 0;
        for (offset, instruction) in instructions.iter().enumerate() {
            match *instruction {
                CONDITIONAL_IF => nesting_level += 1,
                CONDITIONAL_THEN => {
                    if nesting_level == 0 && target == CONDITIONAL_THEN {
                        return Some(start + offset);
                    }
                    nesting_level -= 1;
                }
                CONDITIONAL_ELSE => {
                    if nesting_level == 0 && target == CONDITIONAL_ELSE {
                        return Some(start + offset);
                    }
                }
                _ => {}
            }
        }
        None
    }

    /// Executes a sequence of instructions in the Forth interpreter.
    /// This function takes a definition index and an instruction index,
    /// and executes the instructions starting from that index.
    ///
    /// # Arguments
    ///
    /// - `handler` - The instruction handler that manages the execution of instructions.
    /// - `def_index` - The index of the definition to be executed.
    /// - `instruction_index` - The index of the instruction to start executing from.
    fn execute_instruction<W: Write>(
        &mut self,
        handler: &mut ExecutionHandler<W>,
        def_index: usize,
        instruction_index: usize,
    ) -> Result<(), Error> {
        let mut i = instruction_index;
        while let Some(instruction) = self.definitions.get(def_index).and_then(|def| def.get(i)) {
            match &instruction {
                WordData::DefinitionType(DefinitionType::Name(name)) => {
                    self.execution_stack
                        .push(WordType::UserDefined(name.to_string()));
                }
                WordData::DefinitionIndex(index) => {
                    self.execute_instruction(handler, *index, 0)?;
                }
                WordData::DefinitionType(DefinitionType::If) => {
                    i = self.execute_if(handler, def_index, i)?;
                }
                WordData::DefinitionType(DefinitionType::Else) => {
                    if self.nesting_level > 0 {
                        break;
                    }
                }
                WordData::DefinitionType(DefinitionType::Then) => {
                    self.execute_then()?;
                    if self.nesting_level > 0 {
                        break;
                    }
                }
                _ => handler.handle_word_instruction(instruction)?,
            }
            i += 1;
        }
        Ok(())
    }

    /// Handles the `IF` instruction in the Forth interpreter.
    fn execute_if<W: Write>(
        &mut self,
        handler: &mut ExecutionHandler<W>,
        def_index: usize,
        instruction_index: usize,
    ) -> Result<usize, Error> {
        let then_index =
            self.find_instruction_index(def_index, instruction_index + 1, CONDITIONAL_THEN);
        let else_index =
            self.find_instruction_index(def_index, instruction_index + 1, CONDITIONAL_ELSE);
        let condition = handler.handle_drop_element()?;

        if let Some(then_index) = then_index {
            self.nesting_level += 1;
            if condition == FORTH_TRUE || condition != FORTH_FALSE {
                self.execute_instruction(handler, def_index, instruction_index + 1)?;
            } else if let Some(else_index) = else_index {
                self.execute_instruction(handler, def_index, else_index + 1)?;
            }
            return Ok(then_index);
        }

        Err(ForthError::InvalidWord.into())
    }

    /// Handles the `THEN` instruction in the Forth interpreter.
    fn execute_then(&mut self) -> Result<(), Error> {
        if self.nesting_level > 0 {
            self.nesting_level -= 1;
        }
        Ok(())
    }

    /// Checks if a word is defined in the Forth interpreter.
    pub fn is_word_defined(&self, name: &WordType) -> bool {
        self.words.contains_key(name)
    }

    /// Gets the definition of a word in the Forth interpreter.
    /// This function takes a word name and returns the corresponding word definition.
    /// If the word is not defined, it returns `None`.
    pub fn get_word_definition(&self, name: &WordType) -> Option<&Vec<WordData>> {
        self.words
            .get(name)
            .and_then(|&index| self.definitions.get(index))
    }

    fn is_word_name_valid(&self, name: &str) -> bool {
        if name.parse::<i16>().is_ok() {
            return false;
        }

        name.chars()
            .all(|c| c.is_alphanumeric() || c.is_ascii_punctuation())
    }
}

/// Finds the end of a word definition in the body of instructions.
fn find_end_definition(body: &[Instruction]) -> Option<usize> {
    for (index, element) in body.iter().enumerate() {
        if let Instruction::EndDefinition = element {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forth::boolean_operations::LogicalOperation;
    use crate::forth::intruction::Instruction;
    use std::io::Sink;

    #[test]
    fn can_define_new_words() {
        let mut word_manager = WordDefinitionManager::new();
        // let mut stack = Stack::new(None);
        let data: Vec<Instruction> = vec![
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];
        let expected_result = vec![WordData::Number(-1), WordData::Operator("*".to_string())];

        word_manager
            .define_new_word(WordType::UserDefined("NEGATE".to_string()), data)
            .unwrap();

        assert!(word_manager.is_word_defined(&WordType::UserDefined("NEGATE".to_string())));
        let actual_definition = word_manager
            .get_word_definition(&WordType::UserDefined("NEGATE".to_string()))
            .unwrap();
        assert_eq!(*actual_definition, expected_result);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut word_manager = WordDefinitionManager::new();
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let word: Vec<Instruction> = vec![
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];
        let expected_result = [10];

        let _ = word_manager.define_new_word(WordType::UserDefined("NEGATE".to_string()), word);
        let _ = handler.handle_push_element(-10);
        let _ = word_manager.run_word::<Sink>(&mut handler, "NEGATE");

        assert_eq!(handler.handle_get_stack_content(), &expected_result);
    }

    #[test]
    fn cannot_execute_unknown_word() {
        let mut word_manager = WordDefinitionManager::new();
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let word: Vec<Instruction> = vec![
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];
        let _ = word_manager.define_new_word(WordType::UserDefined("NEGATE".to_string()), word);

        let result = word_manager.run_word::<Sink>(&mut handler, "ABS");

        assert_eq!(result, Err(ForthError::UnknownWord.into()));
    }

    #[test]
    fn can_define_word_that_generate_output() {
        let mut word_manager = WordDefinitionManager::new();
        let word: Vec<Instruction> = vec![
            Instruction::OutpuEmit,
            Instruction::EndDefinition, // end
        ];
        let expected_result = vec![WordData::OutpuEmit];

        let _ = word_manager.define_new_word(WordType::UserDefined("TO-ASCCI".to_string()), word);
        let result =
            word_manager.get_word_definition(&WordType::UserDefined("TO-ASCCI".to_string()));

        assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn run_word_that_generates_output() {
        let mut word_manager = WordDefinitionManager::new();
        let output = Vec::new();
        let mut handler: ExecutionHandler<Vec<u8>> = ExecutionHandler::new(None, Some(output));
        let word: Vec<Instruction> = vec![
            Instruction::OutputDotQuote("Hello".to_string()),
            Instruction::EndDefinition, // end
        ];
        let expected_result = "Hello ".to_string();

        let _ = word_manager.define_new_word(WordType::UserDefined("GREETING".to_string()), word);
        let _ = word_manager.run_word(&mut handler, "GREETING");

        let result = String::from_utf8(handler.handle_get_writer().unwrap().to_vec()).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_define_word_that_contains_conditionals() {
        let mut word_manger = WordDefinitionManager::new();
        let word = vec![
            Instruction::Number(0),
            Instruction::LogicalOperation(LogicalOperation::Equal),
            Instruction::DefinitionType(DefinitionType::If),
            Instruction::OutputDotQuote("Is Zero".to_string()),
            Instruction::DefinitionType(DefinitionType::Then),
            Instruction::EndDefinition,
        ];
        let expected_result = vec![
            WordData::Number(0),
            WordData::LogicalOperation(LogicalOperation::Equal),
            WordData::DefinitionType(DefinitionType::If),
            WordData::OutputDotQuote("Is Zero".to_string()),
            WordData::DefinitionType(DefinitionType::Then),
        ];

        let _ = word_manger.define_new_word(WordType::UserDefined("is-zero?".to_string()), word);
        let result =
            word_manger.get_word_definition(&WordType::UserDefined("is-zero?".to_string()));

        assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn can_execute_word_that_contains_conditionals() {
        let mut word_manager = WordDefinitionManager::new();
        let output = Vec::new();
        let mut handler: ExecutionHandler<Vec<u8>> = ExecutionHandler::new(None, Some(output));
        let word: Vec<Instruction> = vec![
            Instruction::Number(0),
            Instruction::LogicalOperation(LogicalOperation::Equal),
            Instruction::DefinitionType(DefinitionType::If),
            Instruction::OutputDotQuote("Is Zero".to_string()),
            Instruction::DefinitionType(DefinitionType::Else),
            Instruction::OutputDotQuote("Is Not Zero".to_string()),
            Instruction::DefinitionType(DefinitionType::Then),
            Instruction::EndDefinition,
        ];
        let expected_result = "Is Not Zero ".to_string();

        let _ = word_manager.define_new_word(WordType::UserDefined("is-zero?".to_string()), word);
        let _ = handler.handle_push_element(4);
        let _ = word_manager.run_word(&mut handler, "is-zero?");
        let result = String::from_utf8(handler.handle_get_writer().unwrap().to_vec()).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_non_transitive() {
        let mut word_manager = WordDefinitionManager::new();
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let word_foo: Vec<Instruction> = vec![Instruction::Number(5), Instruction::EndDefinition];
        let word_bar: Vec<Instruction> = vec![
            Instruction::DefinitionType(DefinitionType::Name("foo".to_string())),
            Instruction::EndDefinition,
        ];
        let redefinition_foo: Vec<Instruction> =
            vec![Instruction::Number(6), Instruction::EndDefinition];
        let expected_result = vec![5, 6];

        let _ = word_manager.define_new_word(WordType::UserDefined("foo".to_string()), word_foo);
        let _ = word_manager.define_new_word(WordType::UserDefined("bar".to_string()), word_bar);
        let _ = word_manager
            .define_new_word(WordType::UserDefined("foo".to_string()), redefinition_foo);

        let _ = word_manager.run_word::<Sink>(&mut handler, "bar");
        let _ = word_manager.run_word::<Sink>(&mut handler, "foo");

        let result = handler.handle_get_stack_content();

        assert_eq!(result, &expected_result);
    }

    #[test]
    fn test_if_simple() {
        let mut word_manager = WordDefinitionManager::new();
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let word: Vec<Instruction> = vec![
            Instruction::DefinitionType(DefinitionType::If),
            Instruction::Number(2),
            Instruction::DefinitionType(DefinitionType::Then),
            Instruction::EndDefinition,
        ];
        let expected_result = vec![2];

        let _ = word_manager.define_new_word(WordType::UserDefined("f".to_string()), word);
        let _ = handler.handle_push_element(FORTH_TRUE);
        let _ = word_manager.run_word(&mut handler, "f");
        let result = handler.handle_get_stack_content();

        assert_eq!(result, &expected_result);
    }
}
