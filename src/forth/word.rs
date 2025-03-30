use std::collections::HashMap;
use std::io::Write;

use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::forth::forth_errors::ForthError;
use crate::forth::intructions::{DefineWord, ForthData, ForthInstruction};
use crate::stack::stack::Stack;
use crate::stack::stack_operations::execute_stack_operation;

use super::boolean_operations::BooleanOperationManager;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Word {
    Predifined(&'static str),
    UserDefined(String),
}

pub struct WordManager<'a> {
    words: HashMap<Word, Vec<ForthData<'a>>>,
}

impl<'a> WordManager<'a> {
    pub fn new() -> Self {
        WordManager {
            words: HashMap::new(),
        }
    }

    pub fn define_new_word(
        &mut self,
        name: Word,
        body: &'a [ForthInstruction<'a>],
    ) -> Result<(), Error> {
        let end_index = find_end_definition(body).ok_or(ForthError::InvalidWord)?;
        let word_definition = &body[..end_index];
        let mut definition: Vec<ForthData<'a>> = Vec::new();

        for element in word_definition {
            definition.push(self.convert_to_word_defintion(element)?);
        }

        self.words.insert(name, definition);
        Ok(())
    }

    fn convert_to_word_defintion(
        &self,
        instruction: &'a ForthInstruction<'a>,
    ) -> Result<ForthData<'a>, Error> {
        match instruction {
            ForthInstruction::Number(number) => Ok(ForthData::Number(*number)),
            ForthInstruction::Operator(operator) => Ok(ForthData::Operator(operator)),
            ForthInstruction::StackWord(stack_word) => Ok(ForthData::StackWord(stack_word)),
            ForthInstruction::DefineWord(DefineWord::Name(name)) => {
                Ok(ForthData::DefineWord(DefineWord::Name(name.to_string())))
            }
            ForthInstruction::BooleanOperation(boolean_operation) => {
                Ok(ForthData::BooleanOperation(boolean_operation))
            }
            ForthInstruction::LogicalOperation(logical_operation) => {
                Ok(ForthData::LogicalOperation(logical_operation))
            }
            ForthInstruction::OutputDot => Ok(ForthData::OutputDot),
            ForthInstruction::OutpuEmit => Ok(ForthData::OutpuEmit),
            ForthInstruction::OutputCR => Ok(ForthData::OutputCR),
            ForthInstruction::OutputDotQuote(string) => Ok(ForthData::OutputDotQuote(string)),
            _ => Err(ForthError::InvalidWord.into()),
        }
    }

    pub fn execute_word<W: Write>(
        &mut self,
        stack: &mut Stack,
        calculator: &Calculator,
        boolean_manager: &mut BooleanOperationManager,
        mut writer: Option<&mut W>,
        word_name: &str,
    ) -> Result<(), Error> {
        let mut execution_stack = vec![Word::UserDefined(word_name.to_string())];

        while let Some(current_word) = execution_stack.pop() {
            let instructions = match self.words.get(&current_word) {
                Some(definition) => definition,
                None => return Err(ForthError::UnknownWord.into()),
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
                        execution_stack.push(Word::UserDefined(name.to_string()));
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
                    ForthData::OutputDot => {
                        if let Ok(top) = stack.drop() {
                            if let Some(ref mut writer) = writer {
                                println!("{:?}", top);
                                let _ = write!(writer, "{} ", top);
                                let _ = writer.flush();
                            }
                        }
                    }
                    ForthData::OutputCR => {
                        if let Some(ref mut writer) = writer {
                            let _ = writeln!(writer);
                            let _ = writer.flush();
                        }
                    }
                    ForthData::OutpuEmit => {
                        if let Ok(top) = stack.drop() {
                            if let Ok(ascii_char) = u8::try_from(top) {
                                if let Some(ref mut writer) = writer {
                                    println!("{:?}", ascii_char);
                                    let _ = write!(writer, "{} ", ascii_char as char);
                                    let _ = writer.flush();
                                }
                            }
                        }
                    }
                    ForthData::OutputDotQuote(string) => {
                        if let Some(ref mut writer) = writer {
                            let _ = write!(writer, "{} ", string);
                            let _ = writer.flush();
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn is_word_defined(&self, name: &Word) -> bool {
        self.words.contains_key(name)
    }

    pub fn get_word_definition(&self, name: &Word) -> Option<&Vec<ForthData>> {
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
    use std::io::Sink;
    use std::io::Write;

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

        let _ = word_manager
            .define_new_word(Word::UserDefined("NEGATE".to_string()), &data)
            .unwrap();

        assert!(word_manager.is_word_defined(&Word::UserDefined("NEGATE".to_string())));
        let actual_definition = word_manager
            .get_word_definition(&Word::UserDefined("NEGATE".to_string()))
            .unwrap();
        assert_eq!(*actual_definition, expected_result);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![10];

        let _ = word_manager.define_new_word(Word::UserDefined("NEGATE".to_string()), &word);
        let _ = stack.push(-10);
        let _ =
            word_manager.execute_word::<Sink>(stack, &calculator, boolean_manager, None, "NEGATE");

        assert_eq!(stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn cannot_execute_unknown_word() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];
        let _ = word_manager.define_new_word(Word::UserDefined("NEGATE".to_string()), &word);

        let result =
            word_manager.execute_word::<Sink>(stack, &calculator, boolean_manager, None, "ABS");

        assert_eq!(result, Err(ForthError::UnknownWord.into()));
    }

    #[test]
    fn can_define_word_that_generate_output() {
        let mut word_manager = WordManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::OutpuEmit,
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![ForthData::OutpuEmit];

        let _ = word_manager.define_new_word(Word::UserDefined("TO-ASCCI".to_string()), &word);
        let result = word_manager.get_word_definition(&Word::UserDefined("TO-ASCCI".to_string()));

        assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn can_execute_word_that_generates_output() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let mut output = Vec::new();
        let writer: Option<&mut Vec<u8>> = Some(&mut output);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::OutputDotQuote("Hello"),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = "Hello ".to_string();

        let _ = word_manager.define_new_word(Word::UserDefined("GREETING".to_string()), &word);
        let _ = word_manager.execute_word(stack, &calculator, boolean_manager, writer, "GREETING");

        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, expected_result);
    }
}
