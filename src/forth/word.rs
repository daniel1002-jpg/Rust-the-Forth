use std::collections::HashMap;
use std::rc::Rc;

use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::forth::forth_errors::ForthError;
use crate::forth::intructions::{DefineWord, ForthData, ForthInstruction};
use crate::stack::stack::Stack;
use crate::stack::stack_operations::execute_stack_operation;

use super::boolean_operations::BooleanOperationManager;

pub struct WordManager<'a> {
    words: HashMap<&'a str, Rc<Vec<ForthData<'a>>>>,
}

impl<'a> WordManager<'a> {
    pub fn new() -> Self {
        WordManager {
            words: HashMap::new(),
        }
    }

    pub fn define_new_word(
        &mut self,
        name: &'a str,
        body: &[ForthInstruction<'a>],
    ) -> Result<(), Error> {
        let end_index = find_end_definition(body);
        let mut definition = Vec::new();

        if let Some(end_index) = end_index {
            let word_definition = &body[..end_index];
            for element in word_definition {
                match element {
                    ForthInstruction::Number(number) => {
                        definition.push(ForthData::Number(*number));
                    }
                    ForthInstruction::Operator(operator) => {
                        definition.push(ForthData::Operator(operator));
                    }
                    ForthInstruction::StackWord(stack_word) => {
                        definition.push(ForthData::StackWord(stack_word));
                    }
                    ForthInstruction::DefineWord(DefineWord::Name(name)) => {
                        let name = *name;
                        definition.push(ForthData::DefineWord(DefineWord::Name(name)));
                    }
                    ForthInstruction::BooleanOperation(boolean_operation) => {
                        definition.push(ForthData::BooleanOperation(boolean_operation));
                    }
                    ForthInstruction::LogicalOperation(logical_operation) => {
                        definition.push(ForthData::LogicalOperation(logical_operation));
                    }
                    _ => {}
                }
            }
            println!("Data recibed: {:?}", body);
            println!("Data used: {:?}", word_definition);
            println!("Inserting word: {} with definition: {:?}", name, definition);
            self.words.insert(name, Rc::new(definition));
        }
        Ok(())
    }

    pub fn execute_word(
        &mut self,
        stack: &mut Stack,
        calculator: &Calculator,
        boolean_manager: &mut BooleanOperationManager,
        word_name: &'a str,
    ) -> Result<(), Error> {
        let mut execution_stack = vec![word_name];

        while let Some(current_word) = execution_stack.pop() {
            let instructions = match self.words.get(current_word) {
                Some(definition) => Rc::clone(definition),
                // None => println!("{} {}", word_name, Error::ForthError::UnknownWord),
                None => Err(ForthError::UnknownWord)?,
            };

            for element in instructions.iter() {
                match element {
                    ForthData::Number(number) => {
                        stack.push(*number)?;
                    }
                    ForthData::Operator(operator) => {
                        let operand2 = stack.drop()?;
                        let operand1 = stack.drop()?;
                        let result = calculator.calculate(operand2, operand1, operator)?;
                        stack.push(result)?;
                    }
                    ForthData::StackWord(stack_word) => {
                        execute_stack_operation(stack, stack_word)?;
                    }
                    ForthData::DefineWord(DefineWord::Name(name)) => {
                        execution_stack.push(name);
                    }
                    ForthData::BooleanOperation(boolean_operation) => {
                        let operand2 = stack.drop()?;
                        let operand1 = stack.drop()?;
                        let result = boolean_manager.execute_boolean_operation(
                            boolean_operation,
                            operand1,
                            Some(operand2),
                        );
                        stack.push(result)?;
                    }
                    ForthData::LogicalOperation(logical_operation) => {
                        let operand2 = stack.drop()?;
                        let operand1 = stack.drop()?;
                        let result = boolean_manager.execute_logical_operations(
                            logical_operation,
                            operand1,
                            operand2,
                        );
                        stack.push(result)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn is_word_defined(&self, name: &'a str) -> bool {
        self.words.contains_key(name)
    }

    pub fn get_word_definition(&self, name: &'a str) -> Option<&Rc<Vec<ForthData<'a>>>> {
        self.words.get(name)
    }
}

fn find_end_definition(body: &[ForthInstruction<'_>]) -> Option<usize> {
    for (index, element) in body.iter().enumerate() {
        if let ForthInstruction::EndDefinition = element {
            return Some(index);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::forth::intructions::ForthInstruction;
    use crate::stack::stack::Stack;

    #[test]
    fn can_define_new_words() {
        let mut word_manager = WordManager::new();
        // let mut stack = Stack::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![ForthData::Number(-1), ForthData::Operator("*")];

        let _ = word_manager.define_new_word("NEGATE", &data).unwrap();

        assert!(word_manager.is_word_defined("NEGATE"));
        let actual_definition = word_manager.get_word_definition("NEGATE").unwrap();
        assert_eq!(*actual_definition, Rc::new(expected_result));
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator;
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![10];

        let _ = word_manager.define_new_word("NEGATE", &word);
        let _ = stack.push(-10);
        let _ = word_manager.execute_word(stack, &calculator, boolean_manager, "NEGATE");

        assert_eq!(stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn cannot_execute_unknown_word() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator;
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let _ = word_manager.define_new_word("NEGATE", &word);

        let result = word_manager.execute_word(stack, &calculator, boolean_manager, "ABS");

        assert_eq!(result, Err(ForthError::UnknownWord.into()));
    }
}
