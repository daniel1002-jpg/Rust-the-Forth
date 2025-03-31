use super::boolean_operations::BooleanOperationManager;
use super::forth_errors::ForthError;
use super::intructions::*;
use super::word::{Word, WordManager};
use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::stack::stack::Stack;
use crate::stack::stack_operations::{StackOperation, execute_stack_operation};
use std::io::Write;

/// Forth interpreter
/// This struct represents a Forth interpreter with a stack, calculator, and word manager.
/// It allows for the execution of Forth instructions and manipulation of the stack.
/// The `W` type parameter is a generic type that implements the `Write` trait, allowing for
/// output to be directed to different types of writers (e.g., files, stdout).
///
/// # Examples
/// ```
/// use rust_forth::forth::Forth;
/// use std::io::Sink;
/// let forth: Forth<Sink> = Forth::new(None, None);
/// ```
/// # Fields
/// - `stack`: The stack used for storing and manipulating data.
/// - `calculator`: The calculator used for performing arithmetic operations.
/// - `word_manager`: The manager for handling user-defined words.
/// - `boolean_manager`: The manager for handling boolean operations.
/// - `writer`: An optional writer for outputting results.
pub struct Forth<W: Write> {
    stack: Stack,
    calculator: Calculator,
    word_manager: WordManager,
    boolean_manager: BooleanOperationManager,
    writer: Option<W>,
}

impl<W: Write> Forth<W> {
    /// Creates a new instance of the Forth interpreter.
    /// The `stack_capacity` parameter is optional and specifies the initial capacity of the stack. 
    /// The `writer` parameter is also optional and allows for output to be directed to a specific writer.
    /// If no writer is provided, output will be directed to the standard output.
    /// # Examples
    /// ```
    /// use rust_forth::forth::Forth;
    /// use std::io::Sink;
    /// let forth: Forth<Sink> = Forth::new(None, None);
    /// ```
    /// # Arguments
    /// - `stack_capacity`: An optional capacity for the stack.
    /// - `writer`: An optional writer for outputting results.
    pub fn new(stack_capacity: Option<usize>, writer: Option<W>) -> Self {
        Forth {
            stack: Stack::new(stack_capacity),
            calculator: Calculator::new(),
            word_manager: WordManager::new(),
            boolean_manager: BooleanOperationManager::new(),
            writer,
        }
    }

    /// Pushes an element onto the stack.
    pub fn push(&mut self, element: i16) -> Result<(), Error> {
        self.stack.push(element)
    }

    /// Manipulates the stack based on the provided stack operation.
    /// This function executes the specified stack operation (e.g., `dup`, `swap`, `drop`, etc.)
    /// on the stack. It returns an error if the operation fails.
    /// # Arguments
    /// - `stack_operation`: The stack operation to be executed.
    pub fn stack_manipulate(&mut self, stack_operation: &StackOperation) -> Result<(), Error> {
        execute_stack_operation(&mut self.stack, stack_operation)?;
        Ok(())
    }

    /// Gets the current top element of the stack.
    /// This function returns a reference to the top element of the stack.
    /// If the stack is empty, it returns an error.
    pub fn stack_top(&self) -> Result<&i16, Error> {
        self.stack.top()
    }

    /// Calculates the result of an arithmetic operation on two operands.
    fn calculate(&mut self, operator: &str) -> Result<i16, Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;

        let result = self.calculator.calculate(operand1, operand2, operator)?;

        let _ = self.stack.push(result);

        Ok(result)
    }

    /// Processes a vector of Forth instructions.
    /// This function iterates through the provided vector of Forth instructions,
    /// executing each instruction in order. It handles numbers, operators, stack operations,
    /// user-defined words, and boolean operations.
    /// # Arguments
    /// - `data`: A vector of Forth instructions to be processed.
    pub fn process_data(&mut self, data: Vec<ForthInstruction>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            match element {
                &ForthInstruction::Number(number) => {
                    self.stack.push(number)?;
                }
                ForthInstruction::Operator(operator) => {
                    self.calculate(operator)?;
                }
                ForthInstruction::StackWord(stack_word) => {
                    self.stack_manipulate(stack_word)?;
                }
                ForthInstruction::StartDefinition => {
                    self.process_word(data.into_iter().skip(i).collect())?;
                    break;
                }
                ForthInstruction::DefineWord(DefineWord::Name(name)) => {
                    self.execute_new_word(name)?;
                }
                ForthInstruction::BooleanOperation(boolean_operation) => {
                    let operand2 = self.stack.drop()?;
                    let operand1 = self.stack.drop()?;
                    let result = self.boolean_manager.execute_boolean_operation(
                        boolean_operation,
                        operand1,
                        Some(operand2),
                    );
                    self.stack.push(result)?;
                }
                ForthInstruction::LogicalOperation(logical_operation) => {
                    let operand2 = self.stack.drop()?;
                    let operand1 = self.stack.drop()?;
                    let result = self.boolean_manager.execute_logical_operations(
                        logical_operation,
                        operand1,
                        operand2,
                    );
                    self.stack.push(result)?;
                }
                ForthInstruction::OutputDot => {
                    if let Ok(top) = self.stack.drop() {
                        if let Some(writer) = &mut self.writer {
                            println!("{:?}", top);
                            let _ = write!(writer, "{} ", top);
                            let _ = writer.flush();
                        }
                    }
                }
                ForthInstruction::OutputCR => {
                    if let Some(writer) = &mut self.writer {
                        let _ = writeln!(writer);
                        let _ = writer.flush();
                    }
                }
                ForthInstruction::OutpuEmit => {
                    if let Ok(top) = self.stack.drop() {
                        if let Ok(ascii_char) = u8::try_from(top) {
                            if let Some(writer) = &mut self.writer {
                                println!("{:?}", ascii_char);
                                let _ = write!(writer, "{} ", ascii_char as char);
                                let _ = writer.flush();
                            }
                        }
                    }
                }
                ForthInstruction::OutputDotQuote(string) => {
                    if let Some(writer) = &mut self.writer {
                        let _ = write!(writer, "{} ", string);
                        let _ = writer.flush();
                    }
                }
                _ => {}
            }
            println!("{:?}", self.stack);
        }
        Ok(())
    }

    /// Processes a word definition in the Forth interpreter.
    /// This function looks for a word definition in the provided vector of Forth instructions.
    /// If a word definition is found, it extracts the word name and its body,
    /// and defines the new word in the word manager.
    /// # Arguments
    /// - `data`: A vector of Forth instructions containing the word definition.
    fn process_word(&mut self, data: Vec<ForthInstruction>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            if let ForthInstruction::StartDefinition = element {
                if let ForthInstruction::DefineWord(DefineWord::Name(word_name)) = &data[i + 1] {
                    let word_name = Word::UserDefined(word_name.to_string());
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
        word_name: Word,
        word_body: Vec<ForthInstruction>,
    ) -> Result<(), Error> {
        self.word_manager.define_new_word(word_name, word_body)?;
        Ok(())
    }

    /// Executes a new word defined in the Forth interpreter.
    /// This function takes a word name and executes it if it is defined in the word manager.
    /// # Arguments
    /// - `word_name`: The name of the word to be executed.
    fn execute_new_word(&mut self, word_name: &str) -> Result<(), Error> {
        if !self.is_word_defined(&Word::UserDefined(word_name.to_string())) {
            return Err(ForthError::UnknownWord.into());
        }

        self.word_manager.execute_word::<W>(
            &mut self.stack,
            &self.calculator,
            &mut self.boolean_manager,
            self.writer.as_mut(),
            word_name,
        )?;
        Ok(())
    }

    /// Checks if a word is defined in the Forth interpreter.
    fn is_word_defined(&self, word_name: &Word) -> bool {
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
    /// ```
    /// use rust_forth::forth::Forth;
    /// use rust_forth::forth::word::Word;
    /// use std::io::Sink;
    /// let forth: Forth<Sink> = Forth::new(None, None);
    /// let word_name = Word::UserDefined("NEGATE".to_string());
    /// let definition = forth.get_word_definition(&word_name);
    /// assert!(definition.is_some());
    /// ```
    pub fn get_word_definition(&self, word_name: &Word) -> Option<&Vec<ForthData>> {
        self.word_manager.get_word_definition(word_name)
    }

    /// Gets the current content of the stack.
    /// This function returns a reference to the vector of elements currently in the stack.
    /// # Examples
    /// ```
    /// use rust_forth::forth::Forth;
    /// use std::io::Sink;
    /// let forth: Forth<Sink> = Forth::new(None, None);
    /// let elements = vec![1, 2, -3];
    /// for element in &elements {
    ///     let _ = forth.push(*element);
    /// }
    /// assert_eq!(forth.get_stack_content(), &elements);
    /// ```
    /// # Returns
    /// A reference to the vector of elements currently in the stack.
    pub fn get_stack_content(&self) -> &Vec<i16> {
        self.stack.get_stack_content()
    }
}

mod tests {
    #![allow(unused_imports)]
    use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
    use crate::forth::forth::{DefineWord, Forth, ForthData, ForthError, ForthInstruction};
    use crate::forth::word::Word;
    use crate::stack::stack_operations::StackOperation;
    use std::io::Sink;

    #[test]
    fn can_create_forth_with_stack_and_calculator_corectly() {
        let forth: Forth<Sink> = Forth::new(None, None);

        assert!(forth.stack.is_empty());
        assert_eq!(forth.calculator.calculate(2, 4, "+"), Ok(6));
    }

    #[test]
    fn can_push_element_into_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let elements = vec![1, 2, -3];

        for element in &elements {
            let _ = forth.push(*element);
        }

        assert_eq!(forth.stack.size(), 3);
        assert_eq!(forth.stack.top(), Ok(elements.last().unwrap()));
    }

    #[test]
    fn can_be_added_correctly_using_the_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let _ = forth.push(2);
        let _ = forth.push(4);

        assert_eq!(forth.calculate("+"), Ok(6));
    }

    #[test]
    fn can_be_divided_correctly_using_the_stack() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let _ = forth.push(4);
        let _ = forth.push(2);

        assert_eq!(forth.calculate("/"), Ok(2));
    }

    #[test]
    fn can_perform_complex_operations_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let operation: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(2),
            ForthInstruction::Number(4),
            ForthInstruction::Operator("+".to_string()),
            ForthInstruction::Number(6),
            ForthInstruction::Operator("-".to_string()),
            ForthInstruction::Number(8),
            ForthInstruction::Number(2),
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::Number(4),
            ForthInstruction::Operator("/".to_string()),
        ];

        let expected_result = vec![0, 4];
        let _ = forth.process_data(operation);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn stack_can_be_manipulated_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(2),
            ForthInstruction::Number(4),
            ForthInstruction::StackWord(StackOperation::Dup),
            ForthInstruction::StackWord(StackOperation::Rot),
            ForthInstruction::StackWord(StackOperation::Over),
            ForthInstruction::StackWord(StackOperation::Swap),
            ForthInstruction::StackWord(StackOperation::Drop),
        ];
        let expected_result = vec![4, 4, 2];

        let _ = forth.process_data(data);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn can_define_new_words() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];

        let _ = forth.process_data(data);

        assert!(forth.is_word_defined(&Word::UserDefined("NEGATE".to_string())));
        let expected_definition = vec![ForthData::Number(-1), ForthData::Operator("*".to_string())];
        let actual_definition = forth
            .get_word_definition(&&Word::UserDefined("NEGATE".to_string()))
            .unwrap();

        assert_eq!(*actual_definition, expected_definition);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-10),
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
        ];
        let expected_result = vec![10];

        let _ = forth.process_data(word);
        let _ = forth.process_data(data);

        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn cannot_define_invalid_word() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::Number(11),
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];

        let result = forth.process_data(data);

        assert_eq!(result, Err(ForthError::InvalidWord.into()));
    }

    #[test]
    fn can_execute_boolean_operations_correctly() {
        let mut forth: Forth<Sink> = Forth::new(None, None);
        let data = vec![
            ForthInstruction::Number(3),
            ForthInstruction::Number(4),
            ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
            ForthInstruction::Number(20),
            ForthInstruction::Number(10),
            ForthInstruction::LogicalOperation(LogicalOperation::GreaterThan),
            ForthInstruction::BooleanOperation(BooleanOperation::And),
        ];

        let expected_result = vec![-1];

        assert_eq!(forth.process_data(data), Ok(()));
        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn can_execute_output_instructions_correctly() {
        let output = Vec::new();
        let mut forth = Forth::new(None, Some(output));
        let instruction = vec![
            ForthInstruction::Number(3),
            ForthInstruction::OutputDot,
            ForthInstruction::Number(65),
            ForthInstruction::OutpuEmit,
            ForthInstruction::Number(4),
            ForthInstruction::OutputCR,
            ForthInstruction::OutputDotQuote("word".to_string()),
        ];
        let expected_result = "3 A \nword ";

        let _ = forth.process_data(instruction);

        let result = String::from_utf8(forth.writer.unwrap()).unwrap();

        assert_eq!(result, expected_result);
    }
}
