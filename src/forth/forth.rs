// use std::rc::Rc;

use super::boolean_operations::BooleanOperationManager;
use super::forth_errors::ForthError;
use super::intructions::*;
use super::word::{Word, WordManager};
use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::stack::stack::Stack;
use crate::stack::stack_operations::{StackOperation, execute_stack_operation};

pub struct Forth<'a> {
    stack: Stack,
    calculator: Calculator,
    word_manager: WordManager<'a>,
    boolean_manager: BooleanOperationManager,
}

impl<'a> Forth<'a> {
    pub fn new(stack_capacity: Option<usize>) -> Self {
        Forth {
            stack: Stack::new(stack_capacity),
            calculator: Calculator::new(),
            word_manager: WordManager::new(),
            boolean_manager: BooleanOperationManager::new(),
        }
    }

    pub fn push(&mut self, element: i16) -> Result<(), Error> {
        self.stack.push(element)
    }

    pub fn stack_manipulate(&mut self, stack_operation: &StackOperation) -> Result<(), Error> {
        execute_stack_operation(&mut self.stack, stack_operation)?;
        Ok(())
    }

    pub fn stack_top(&self) -> Result<&i16, Error> {
        self.stack.top()
    }

    fn calculate(&mut self, operator: &str) -> Result<i16, Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;

        let result = self.calculator.calculate(operand1, operand2, operator)?;

        let _ = self.stack.push(result);

        Ok(result)
    }

    pub fn process_data(&mut self, data: &'a Vec<ForthInstruction<'a>>) -> Result<(), Error> {
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
                    self.process_word(&data[i..])?;
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
                _ => {}
            }
            println!("{:?}", self.stack);
        }
        Ok(())
    }

    fn process_word(&mut self, data: &'a [ForthInstruction<'a>]) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            if let ForthInstruction::StartDefinition = element {
                if let ForthInstruction::DefineWord(DefineWord::Name(word_name)) = &data[i + 1] {
                    let word_name = Word::UserDefined(word_name.to_string());
                    let _ = self.define_new_word(word_name, &data[i + 2..]);
                    break;
                } else {
                    return Err(ForthError::InvalidWord.into());
                }
            }
        }
        Ok(())
    }

    fn define_new_word(
        &mut self,
        word_name: Word,
        word_body: &'a [ForthInstruction<'a>],
    ) -> Result<(), Error> {
        self.word_manager.define_new_word(word_name, &word_body)?;
        Ok(())
    }

    fn execute_new_word(&mut self, word_name: &str) -> Result<(), Error> {
        self.word_manager.execute_word(
            &mut self.stack,
            &self.calculator,
            &mut self.boolean_manager,
            word_name,
        )?;
        Ok(())
    }

    fn is_word_defined(&self, word_name: &Word) -> bool {
        self.word_manager.is_word_defined(word_name)
    }

    pub fn get_word_definition(&self, word_name: &Word) -> Option<&Vec<ForthData>> {
        self.word_manager.get_word_definition(word_name)
    }
}

mod tests {
    // use std::rc::Rc;

    use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
    use crate::forth::forth::{DefineWord, Forth, ForthData, ForthError, ForthInstruction};
    use crate::forth::word::Word;
    use crate::stack::stack_operations::StackOperation;

    #[test]
    fn can_create_forth_with_stack_and_calculator_corectly() {
        let forth = Forth::new(None);

        assert!(forth.stack.is_empty());
        assert_eq!(forth.calculator.calculate(2, 4, "+"), Ok(6));
    }

    #[test]
    fn can_push_element_into_stack() {
        let mut forth = Forth::new(None);
        let elements = vec![1, 2, -3];

        for element in &elements {
            let _ = forth.push(*element);
        }

        assert_eq!(forth.stack.size(), 3);
        assert_eq!(forth.stack.top(), Ok(elements.last().unwrap()));
    }

    #[test]
    fn can_be_added_correctly_using_the_stack() {
        let mut forth = Forth::new(None);
        let _ = forth.push(2);
        let _ = forth.push(4);

        assert_eq!(forth.calculate("+"), Ok(6));
    }

    #[test]
    fn can_be_divided_correctly_using_the_stack() {
        let mut forth = Forth::new(None);
        let _ = forth.push(4);
        let _ = forth.push(2);

        assert_eq!(forth.calculate("/"), Ok(2));
    }

    #[test]
    fn can_perform_complex_operations_correctly() {
        let mut forth = Forth::new(None);
        let operation: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(2),
            ForthInstruction::Number(4),
            ForthInstruction::Operator("+"),
            ForthInstruction::Number(6),
            ForthInstruction::Operator("-"),
            ForthInstruction::Number(8),
            ForthInstruction::Number(2),
            ForthInstruction::Operator("*"),
            ForthInstruction::Number(4),
            ForthInstruction::Operator("/"),
        ];

        let expected_result = vec![0, 4];
        let _ = forth.process_data(&operation);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn stack_can_be_manipulated_correctly() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(2),
            ForthInstruction::Number(4),
            ForthInstruction::StackWord(&StackOperation::Dup),
            ForthInstruction::StackWord(&StackOperation::Rot),
            ForthInstruction::StackWord(&StackOperation::Over),
            ForthInstruction::StackWord(&StackOperation::Swap),
            ForthInstruction::StackWord(&StackOperation::Drop),
        ];
        let expected_result = vec![4, 4, 2];

        let _ = forth.process_data(&data);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn can_define_new_words() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];

        let _ = forth.process_data(&data);

        assert!(forth.is_word_defined(&Word::UserDefined("NEGATE".to_string())));
        let expected_definition = vec![ForthData::Number(-1), ForthData::Operator("*")];
        let actual_definition = forth
            .get_word_definition(&&Word::UserDefined("NEGATE".to_string()))
            .unwrap();

        assert_eq!(*actual_definition, expected_definition);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut forth = Forth::new(None);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-10),
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())), // word
        ];
        let expected_result = vec![10];

        let _ = forth.process_data(&word);
        let _ = forth.process_data(&data);

        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn cannot_define_invalid_word() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition, // start
            ForthInstruction::Number(11),
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];

        let result = forth.process_data(&data);

        assert_eq!(result, Err(ForthError::InvalidWord.into()));
    }

    #[test]
    fn can_execute_boolean_operations_correctly() {
        let mut forth = Forth::new(None);
        let data = vec![
            ForthInstruction::Number(3),
            ForthInstruction::Number(4),
            ForthInstruction::LogicalOperation(&LogicalOperation::LessThan),
            ForthInstruction::Number(20),
            ForthInstruction::Number(10),
            ForthInstruction::LogicalOperation(&LogicalOperation::GreaterThan),
            ForthInstruction::BooleanOperation(&BooleanOperation::And),
        ];

        let expected_result = vec![-1];

        assert_eq!(forth.process_data(&data), Ok(()));
        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }
}
