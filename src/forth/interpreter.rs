use super::definition_type::DefinitionType;
use super::forth_errors::ForthError;
use super::intruction::Instruction;
use super::parser::Parser;
use super::word::{WordDefinitionManager, WordType};
use super::word_data::WordData;
use crate::errors::Error;
use crate::handler::instructions_handler::ExecutionHandler;
use std::io::Write;

/// Forth interpreter
/// This struct represents a Forth interpreter with a stack, calculator, and word manager.
/// It allows for the execution of Forth instructions and manipulation of the stack.
/// The `W` type parameter is a generic type that implements the `Write` trait, allowing for
/// output to be directed to different types of writers (e.g., files, stdout).
///
/// # Examples
/// ```
///# use rust_forth::forth::interpreter::Forth;
///# use std::io::Sink;
/// let forth: Forth<Sink> = Forth::new(None, None);
/// ```
/// # Fields
/// - `stack`: The stack used for storing and manipulating data.
/// - `calculator`: The calculator used for performing arithmetic operations.
/// - `word_manager`: The manager for handling user-defined words.
/// - `boolean_manager`: The manager for handling boolean operations.
/// - `writer`: An optional writer for outputting results.
/// - `parser`: The parser used for interpreting Forth instructions.
pub struct Forth<W: Write> {
    handler: ExecutionHandler<W>,
    word_manager: WordDefinitionManager,
    parser: Parser,
}

impl<W: Write> Forth<W> {
    /// Creates a new instance of the Forth interpreter.
    /// The `stack_capacity` parameter is optional and specifies the initial capacity of the stack.
    /// The `writer` parameter is also optional and allows for output to be directed to a specific writer.
    /// If no writer is provided, output will be directed to the standard output.
    /// # Examples
    /// ```
    ///# use rust_forth::forth::interpreter::Forth;
    ///# use std::io::Sink;
    /// let forth: Forth<Sink> = Forth::new(None, None);
    /// ```
    /// # Arguments
    /// - `stack_capacity`: An optional capacity for the stack.
    /// - `writer`: An optional writer for outputting results.
    pub fn new(stack_capacity: Option<usize>, writer: Option<W>) -> Self {
        Forth {
            word_manager: WordDefinitionManager::new(),
            handler: ExecutionHandler::new(stack_capacity, writer),
            parser: Parser::new(),
        }
    }

    /// Pushes an element onto the stack.
    pub fn push(&mut self, element: i16) -> Result<(), Error> {
        self.handler.handle_push_element(element)
    }

    /// Gets the current top element of the stack.
    /// This function returns a reference to the top element of the stack.
    /// If the stack is empty, it returns an error.
    pub fn peek_stack(&mut self) -> Result<&i16, Error> {
        self.handler.handle_get_top_element()
    }

    /// Processes a vector of Forth instructions.
    /// This function iterates through the provided vector of Forth instructions,
    /// executing each instruction in order. It handles numbers, operators, stack operations,
    /// user-defined words, and boolean operations.
    /// # Arguments
    /// - `data`: A vector of Forth instructions to be processed.
    pub fn process_instructions(&mut self, data: Vec<Instruction>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            match element {
                Instruction::StartDefinition => {
                    self.define_word(data.into_iter().skip(i).collect())?;
                    break;
                }
                Instruction::DefinitionType(DefinitionType::Name(name)) => {
                    self.execute_new_word(name)?;
                }
                _ => self.handler.handle_instruction(element)?,
            }
        }
        Ok(())
    }

    /// Processes a word definition in the Forth interpreter.
    /// This function looks for a word definition in the provided vector of Forth instructions.
    /// If a word definition is found, it extracts the word name and its body,
    /// and defines the new word in the word manager.
    /// # Arguments
    /// - `data`: A vector of Forth instructions containing the word definition.
    fn define_word(&mut self, data: Vec<Instruction>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            if let Instruction::StartDefinition = element {
                if let Instruction::DefinitionType(DefinitionType::Name(word_name)) = &data[i + 1] {
                    let word_name = WordType::UserDefined(word_name.to_string());
                    self.define_new_word(word_name, data.into_iter().skip(i + 2).collect())?;
                    break;
                } else {
                    return Err(ForthError::InvalidWord.into());
                }
            }
        }
        Ok(())
    }

    /// Defines a new word in the Forth interpreter.
    /// This function takes a word name and its body (a vector of Forth instructions),
    /// and defines the new word in the word manager.
    /// # Arguments
    /// - `word_name`: The name of the new word to be defined.
    /// - `word_body`: A vector of Forth instructions representing the body of the new word.    
    fn define_new_word(
        &mut self,
        word_name: WordType,
        word_body: Vec<Instruction>,
    ) -> Result<(), Error> {
        self.word_manager.define_new_word(word_name, word_body)?;
        Ok(())
    }

    /// Executes a new word defined in the Forth interpreter.
    /// This function takes a word name and executes it if it is defined in the word manager.
    /// # Arguments
    /// - `word_name`: The name of the word to be executed.
    fn execute_new_word(&mut self, word_name: &str) -> Result<(), Error> {
        if !self.is_word_defined(&WordType::UserDefined(word_name.to_string())) {
            return Err(ForthError::UnknownWord.into());
        }

        self.word_manager.run_word(&mut self.handler, word_name)?;
        Ok(())
    }

    /// Checks if a word is defined in the Forth interpreter.
    pub fn is_word_defined(&self, word_name: &WordType) -> bool {
        self.word_manager.is_word_defined(word_name)
    }

    /// Gets the definition of a word in the Forth interpreter.
    /// This function takes a word name and returns its definition if it is defined in the word manager.
    /// # Arguments
    /// - `word_name`: The name of the word whose definition is to be retrieved.
    ///
    /// Returns an `Option` containing a reference to the vector of Forth data representing the word's definition.
    ///
    /// If the word is not defined, it returns `None`.
    /// # Examples
    /// ```rust
    ///# use rust_forth::forth::interpreter::Forth;
    ///# use rust_forth::forth::intruction::Instruction;
    ///# use rust_forth::forth::word_data::WordData;
    ///# use rust_forth::forth::definition_type::DefinitionType;
    ///# use rust_forth::forth::word::WordType;
    ///# use std::io::Sink;
    ///
    /// let mut forth: Forth<Sink> = Forth::new(None, None);
    /// let data: Vec<Instruction> = vec![
    ///     Instruction::StartDefinition, // start
    ///     Instruction::DefinitionType(DefinitionType::Name("NEGATE".to_string())), // word
    ///     Instruction::Number(-1),
    ///     Instruction::Operator("*".to_string()),
    ///     Instruction::EndDefinition, // end
    /// ];
    ///
    /// let _ = forth.process_instructions(data);
    ///
    /// assert!(forth.is_word_defined(&WordType::UserDefined("NEGATE".to_string())));
    /// let expected_definition = vec![
    ///     WordData::Number(-1),
    ///     WordData::Operator("*".to_string()),
    /// ];
    /// let actual_definition = forth
    ///     .fetch_word_definition(&WordType::UserDefined("NEGATE".to_string()))
    ///     .unwrap();
    ///
    /// assert_eq!(*actual_definition, expected_definition);
    /// ```
    pub fn fetch_word_definition(&mut self, word_name: &WordType) -> Option<&Vec<WordData>> {
        self.word_manager.get_word_definition(word_name)
    }

    /// Gets the current content of the stack.
    /// This function returns a reference to the vector of elements currently in the stack.
    /// # Examples
    /// ```
    /// use rust_forth::forth::interpreter::Forth;
    /// use std::io::Sink;
    /// let mut forth: Forth<Sink> = Forth::new(None, None);
    /// let elements = vec![1, 2, -3];
    /// for element in &elements {
    ///     let _ = forth.push(*element);
    /// }
    /// assert_eq!(forth.get_stack_content(), &elements);
    /// ```
    /// # Returns
    /// A reference to the vector of elements currently in the stack.
    pub fn get_stack_content(&self) -> &Vec<i16> {
        self.handler.handle_get_stack_content()
    }

    /// Parses a line of Forth instructions.
    /// This function takes a line of text and parses it into a vector of Forth instructions.
    ///
    /// # Examples
    /// ```rust
    /// use rust_forth::forth::interpreter::Forth;
    /// use rust_forth::forth::intruction::Instruction;
    /// use std::io::Sink;
    ///
    /// let forth: Forth<Sink> = Forth::new(None, None);
    /// let line = "1 2 3 . . .";
    /// let expected_instructions = vec![
    ///    Instruction::Number(1),
    ///    Instruction::Number(2),
    ///    Instruction::Number(3),
    ///    Instruction::OutputDot,
    ///    Instruction::OutputDot,
    ///    Instruction::OutputDot,
    /// ];
    ///
    /// let instructions = forth.parse_instructions(line.to_string());
    ///
    /// assert_eq!(instructions, expected_instructions);
    /// ```
    pub fn parse_instructions(&self, line: String) -> Vec<Instruction> {
        self.parser.parse_instructions(line, &self.word_manager)
    }

    /// Checks if the stack is empty.
    pub fn is_stack_empty(&self) -> bool {
        self.handler.handle_is_empty()
    }

    /// Returns the size of the stack.
    /// This function returns the number of elements currently in the stack.
    pub fn stack_size(&self) -> usize {
        self.handler.handle_stack_size()
    }

    /// Returns a writer for output (if exists).
    /// This function returns a mutable reference to the writer used for output.
    pub fn get_writer(&mut self) -> Option<&mut W> {
        self.handler.handle_get_writer()
    }
}

#[cfg(test)]
mod tests {
    // #![allow(unused_imports)]
    use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
    use crate::forth::interpreter::{DefinitionType, Forth, ForthError, Instruction, WordData};
    use crate::forth::word::WordType;
    use crate::stack::stack_operations::StackOperation;
    use std::io::Sink;
    #[test]
    fn can_create_forth_with_stack_and_calculator_corectly() {
        let forth: Forth<Sink> = Forth::new(None, None);

        assert!(forth.handler.handle_is_empty());
        // assert_eq!(forth.handler.handle_calculate(2, 4, "+"), Ok(6));
    }

    #[test]
    fn can_push_element_into_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let elements = vec![1, 2, -3];

        for element in &elements {
            let _ = forth.push(*element);
        }

        assert_eq!(forth.stack_size(), 3);
        assert_eq!(forth.peek_stack(), Ok(elements.last().unwrap()));
    }

    #[test]
    fn can_be_added_correctly_using_the_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let _ = forth.push(2);
        let _ = forth.push(4);
        let operation = Instruction::Operator("+".to_string());
        let expected_result = vec![6];

        let _ = forth.handler.handle_instruction(&operation);

        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn can_be_divided_correctly_using_the_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let _ = forth.push(4);
        let _ = forth.push(2);
        let operation = Instruction::Operator("/".to_string());
        let expected_result = vec![2];

        let _ = forth.handler.handle_instruction(&operation);

        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn can_perform_complex_operations_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let operation: Vec<Instruction> = vec![
            Instruction::Number(2),
            Instruction::Number(4),
            Instruction::Operator("+".to_string()),
            Instruction::Number(6),
            Instruction::Operator("-".to_string()),
            Instruction::Number(8),
            Instruction::Number(2),
            Instruction::Operator("*".to_string()),
            Instruction::Number(4),
            Instruction::Operator("/".to_string()),
        ];

        let expected_result = [0, 4];
        let _ = forth.process_instructions(operation);

        assert_eq!(forth.stack_size(), expected_result.len());
        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn stack_can_be_manipulated_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<Instruction> = vec![
            Instruction::Number(2),
            Instruction::Number(4),
            Instruction::StackWord(StackOperation::Dup),
            Instruction::StackWord(StackOperation::Rot),
            Instruction::StackWord(StackOperation::Over),
            Instruction::StackWord(StackOperation::Swap),
            Instruction::StackWord(StackOperation::Drop),
        ];
        let expected_result = vec![4, 4, 4];

        let _ = forth.process_instructions(data);

        assert_eq!(forth.stack_size(), expected_result.len());
        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn can_define_new_words() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<Instruction> = vec![
            Instruction::StartDefinition, // start
            Instruction::DefinitionType(DefinitionType::Name("NEGATE".to_string())), // word
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];

        let _ = forth.process_instructions(data);

        assert!(forth.is_word_defined(&WordType::UserDefined("NEGATE".to_string())));
        let expected_definition = vec![WordData::Number(-1), WordData::Operator("*".to_string())];
        let actual_definition = forth
            .fetch_word_definition(&WordType::UserDefined("NEGATE".to_string()))
            .unwrap();

        assert_eq!(*actual_definition, expected_definition);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let word: Vec<Instruction> = vec![
            Instruction::StartDefinition, // start
            Instruction::DefinitionType(DefinitionType::Name("NEGATE".to_string())), // word
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];
        let data: Vec<Instruction> = vec![
            Instruction::Number(-10),
            Instruction::DefinitionType(DefinitionType::Name("NEGATE".to_string())), // word
        ];
        let expected_result = [10];

        let _ = forth.process_instructions(word);
        let _ = forth.process_instructions(data);

        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn cannot_define_invalid_word() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<Instruction> = vec![
            Instruction::StartDefinition, // start
            Instruction::Number(11),
            Instruction::Number(-1),
            Instruction::Operator("*".to_string()),
            Instruction::EndDefinition, // end
        ];

        let result = forth.process_instructions(data);

        assert_eq!(result, Err(ForthError::InvalidWord.into()));
    }

    #[test]
    fn can_execute_boolean_operations_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data = vec![
            Instruction::Number(3),
            Instruction::Number(4),
            Instruction::LogicalOperation(LogicalOperation::LessThan),
            Instruction::Number(20),
            Instruction::Number(10),
            Instruction::LogicalOperation(LogicalOperation::GreaterThan),
            Instruction::BooleanOperation(BooleanOperation::And),
        ];

        let expected_result = [-1];

        assert_eq!(forth.process_instructions(data), Ok(()));
        assert_eq!(forth.get_stack_content(), &expected_result);
    }

    #[test]
    fn can_execute_output_instructions_correctly() {
        let output = Vec::new();
        let mut forth = Forth::new(None, Some(output));
        let instruction = vec![
            Instruction::Number(3),
            Instruction::OutputDot,
            Instruction::Number(65),
            Instruction::OutpuEmit,
            Instruction::Number(4),
            Instruction::OutputCR,
            Instruction::OutputDotQuote("word".to_string()),
        ];
        let expected_result = "3 A \nword ";

        let _ = forth.process_instructions(instruction);

        let result = String::from_utf8(forth.get_writer().unwrap().to_vec()).unwrap();

        assert_eq!(result, expected_result);
    }
}
