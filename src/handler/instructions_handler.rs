use std::io::Write;

use crate::{
    BooleanOperation, Instruction, LogicalOperation, Stack,
    calculator::operations::Calculator,
    errors::Error,
    forth::{
        boolean_operations::BooleanOperationManager,
        output_instructions::{CR, DOT, EMIT, OutputInstruction},
        word_data::WordData,
    },
    stack::stack_operations::execute_stack_operation,
};

/// # ExecutionHandler struct
///
/// This struct is responsible for handling the instructions around of the interpreter.
///
/// ## Fields
///
/// - `stack`: The stack used to store the values.
/// - `calculator`: The calculator used to perform arithmetic operations.
/// - `boolean_manager`: The boolean manager used to manage the boolean operations.
/// - `writer`: The writer used to write the output.
///
/// ## Principal Methods
///
/// - `new`: Creates a new instance of the ExecutionHandler.
/// - `handle_instruction`: Handles the instructions of the Forth interpreter.
/// - `handle_word_instruction`: Handles the word instructions.
/// - `handle_get_writer`: Returns a mutable reference to the writer.
/// - `handle_get_top_element`: Returns a reference to the top element of the stack.
/// - `handle_push_element`: Pushes an element onto the stack.
/// - `handle_drop_element`: Drops the top element from the stack.
/// - `handle_get_stack_content`: Returns a reference to the stack content.
/// - `handle_is_empty`: Checks if the stack is empty.
/// - `handle_stack_size`: Returns the size of the stack.
pub struct ExecutionHandler<W: Write> {
    stack: Stack,
    calculator: Calculator,
    boolean_manager: BooleanOperationManager,
    writer: Option<W>,
}

impl<W: Write> ExecutionHandler<W> {
    /// Creates a new instance of the ExecutionHandler.
    ///
    /// # Arguments
    ///
    /// - `stack_capacity`: The capacity of the stack.
    /// - `writer`: An optional writer for outputting results.
    pub fn new(stack_capacity: Option<usize>, writer: Option<W>) -> Self {
        ExecutionHandler {
            stack: Stack::new(stack_capacity),
            calculator: Calculator::new(),
            boolean_manager: BooleanOperationManager::new(),
            writer,
        }
    }

    /// Handles the instructions of the Forth interpreter.
    ///
    /// In this method, the instructions are processed one by one.
    /// The method checks the type of each instruction and calls the appropriate handler method.
    pub fn handle_instruction(&mut self, instruction: &Instruction) -> Result<(), Error> {
        match instruction {
            &Instruction::Number(number) => self.handle_push_element(number)?,
            Instruction::Operator(operator) => self.handle_calculate(operator)?,
            Instruction::StackWord(stack_word) => {
                execute_stack_operation(&mut self.stack, stack_word)?
            }
            Instruction::BooleanOperation(boolean_operation) => {
                self.handle_boolean_operation(boolean_operation)?
            }
            Instruction::LogicalOperation(logical_operation) => {
                self.handle_logical_operation(logical_operation)?
            }
            _ => self.handle_generation_output(instruction)?,
        }
        Ok(())
    }

    /// Handles the word instructions.
    ///
    /// This method is responsible for processing the word instructions.
    /// It checks the type of each word instruction and calls the appropriate handler method.
    pub fn handle_word_instruction(&mut self, instruction: &WordData) -> Result<(), Error> {
        match instruction {
            &WordData::Number(number) => self.handle_push_element(number)?,
            WordData::Operator(operator) => self.handle_calculate(operator)?,
            WordData::StackWord(stack_word) => {
                execute_stack_operation(&mut self.stack, stack_word)?
            }
            WordData::BooleanOperation(boolean_operation) => {
                self.handle_boolean_operation(boolean_operation)?
            }
            WordData::LogicalOperation(logical_operation) => {
                self.handle_logical_operation(logical_operation)?
            }
            WordData::Output(DOT) => self.handle_output_dot()?,
            WordData::Output(CR) => self.handle_output_cr()?,
            WordData::Output(EMIT) => self.handle_output_emit()?,
            WordData::Output(OutputInstruction::DotQuote(str)) => {
                self.handle_output_dot_quote(str)?
            }
            _ => {}
        }
        Ok(())
    }

    /// Returns a mutable reference to the writer, if it exists.
    pub fn handle_get_writer(&mut self) -> Option<&mut W> {
        self.writer.as_mut()
    }

    /// Returns a reference to the top element of the stack.
    /// If the stack is empty, it returns an error.
    pub fn handle_get_top_element(&mut self) -> Result<&i16, Error> {
        self.stack.top()
    }

    /// Pushes an element onto the stack.
    /// If the stack is full, it returns an error.
    pub fn handle_push_element(&mut self, element: i16) -> Result<(), Error> {
        self.stack.push(element)?;
        Ok(())
    }

    /// Drops the top element from the stack.
    /// If the stack is empty, it returns an error.
    pub fn handle_drop_element(&mut self) -> Result<i16, Error> {
        self.stack.drop()
    }

    /// Returns a reference to the stack content.
    pub fn handle_get_stack_content(&self) -> &Vec<i16> {
        self.stack.get_stack_content()
    }

    /// Checks if the stack is empty.
    pub fn handle_is_empty(&self) -> bool {
        self.stack.is_empty()
    }

    /// Returns the size of the stack.
    pub fn handle_stack_size(&self) -> usize {
        self.stack.size()
    }

    /// Handles the calculation operations.
    fn handle_calculate(&mut self, operation: &str) -> Result<(), Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;
        let result = self.calculator.calculate(operand1, operand2, operation)?;
        self.stack.push(result)?;
        Ok(())
    }

    /// Handles the boolean operations.
    fn handle_boolean_operation(&mut self, operation: &BooleanOperation) -> Result<(), Error> {
        let operand1 = self.stack.drop()?;
        let operand2 = if self.boolean_manager.is_not(operation) {
            None
        } else {
            Some(self.stack.drop()?)
        };
        let result = self
            .boolean_manager
            .execute_boolean_operation(operation, operand1, operand2);
        self.stack.push(result)?;
        Ok(())
    }

    /// Handles the logical operations.
    fn handle_logical_operation(&mut self, operation: &LogicalOperation) -> Result<(), Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;
        let result = self
            .boolean_manager
            .execute_logical_operations(operation, operand1, operand2);
        self.stack.push(result)?;
        Ok(())
    }

    /// Handles the generation output instructions.
    fn handle_generation_output(&mut self, instruction: &Instruction) -> Result<(), Error> {
        match instruction {
            Instruction::Output(DOT) => self.handle_output_dot()?,
            Instruction::Output(CR) => self.handle_output_cr()?,
            Instruction::Output(EMIT) => self.handle_output_emit()?,
            Instruction::Output(OutputInstruction::DotQuote(str)) => {
                self.handle_output_dot_quote(str)?
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles the output dot instruction.
    fn handle_output_dot(&mut self) -> Result<(), Error> {
        if let Ok(top) = self.stack.drop() {
            if let Some(writer) = &mut self.writer {
                let _ = write!(writer, "{} ", top);
                let _ = writer.flush();
            }
        }
        Ok(())
    }

    /// Handles the output carriage return instruction.
    fn handle_output_cr(&mut self) -> Result<(), Error> {
        if let Some(writer) = &mut self.writer {
            let _ = writeln!(writer);
            let _ = writer.flush();
        }
        Ok(())
    }

    /// Handles the output emit instruction.
    fn handle_output_emit(&mut self) -> Result<(), Error> {
        if let Ok(top) = self.stack.drop() {
            if let Ok(ascii_char) = u8::try_from(top) {
                if let Some(writer) = &mut self.writer {
                    let _ = write!(writer, "{} ", ascii_char as char);
                    let _ = writer.flush();
                }
            }
        }
        Ok(())
    }

    /// Handles the output dot quote instruction.
    fn handle_output_dot_quote(&mut self, string: &str) -> Result<(), Error> {
        if let Some(writer) = &mut self.writer {
            let _ = write!(writer, "{} ", string);
            let _ = writer.flush();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::stack_operations::{DROP, DUP, OVER, ROT, SWAP};
    use std::io::Sink;

    #[test]
    fn test_handle_push_element() {
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let element = Instruction::Number(5);
        let expected_result = vec![5];

        handler.handle_instruction(&element).unwrap();

        assert_eq!(handler.stack.get_stack_content(), &expected_result);
    }

    #[test]
    fn test_handle_calculate() {
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let expected_result = vec![8];

        handler.stack.push(5).unwrap();
        handler.stack.push(3).unwrap();

        handler.handle_calculate("+").unwrap();

        assert_eq!(handler.stack.get_stack_content(), &expected_result);
    }

    #[test]
    fn test_handle_manipulate_stack() {
        let mut handler: ExecutionHandler<Sink> = ExecutionHandler::new(None, None);
        let instructions: Vec<Instruction> = vec![
            Instruction::number(2),
            Instruction::number(4),
            Instruction::stack_word(DUP),
            Instruction::stack_word(ROT),
            Instruction::stack_word(OVER),
            Instruction::stack_word(SWAP),
            Instruction::stack_word(DROP),
        ];
        let expected_result = vec![4, 4, 4];

        for instruction in instructions {
            handler.handle_instruction(&instruction).unwrap();
        }

        assert_eq!(handler.stack.get_stack_content(), &expected_result);
    }
}
