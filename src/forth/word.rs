use std::collections::HashMap;
use std::io::Write;

use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::forth::forth_errors::ForthError;
use crate::forth::intructions::{DefineWord, ForthData, ForthInstruction};
use crate::stack::stack::Stack;
use crate::stack::stack_operations::execute_stack_operation;

use super::boolean_operations::{BooleanOperationManager, TRUE};

/// Enum that represents a word in the Forth language.
/// It can be either a predefined word (like "DUP") or a user-defined word (like "MY_WORD").
/// The predefined words are represented as static strings, while the user-defined words are
/// represented as owned strings.
/// The `Word` enum is used to identify the type of word being defined or executed.
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Word {
    Predifined(&'static str),
    UserDefined(String),
}

/// Struct that represents a word manager in the Forth interpreter
/// 
pub struct WordManager {
    words: HashMap<Word, Vec<ForthData>>,
}

impl Default for WordManager {
    fn default() -> Self {
        WordManager::new()
    }
}

impl WordManager {
    pub fn new() -> Self {
        WordManager {
            words: HashMap::new(),
        }
    }

    pub fn define_new_word(
        &mut self,
        name: Word,
        body: Vec<ForthInstruction>,
    ) -> Result<(), Error> {
        let end_index = find_end_definition(&body).ok_or(ForthError::InvalidWord)?;
        let word_definition = body.into_iter().take(end_index).collect::<Vec<_>>();
        // let word_definition = &body[..end_index];
        let mut definition: Vec<ForthData> = Vec::new();

        for element in word_definition {
            definition.push(self.convert_to_word_defintion(element)?);
        }

        self.words.insert(name, definition);
        Ok(())
    }

    fn convert_to_word_defintion(&self, instruction: ForthInstruction) -> Result<ForthData, Error> {
        match instruction {
            ForthInstruction::Number(number) => Ok(ForthData::Number(number)),
            ForthInstruction::Operator(operator) => Ok(ForthData::Operator(String::from(operator))),
            ForthInstruction::StackWord(stack_word) => Ok(ForthData::StackWord(stack_word)),
            ForthInstruction::DefineWord(define_word) => match define_word {
                DefineWord::Name(name) => {
                    Ok(ForthData::DefineWord(DefineWord::Name(name.to_string())))
                }
                DefineWord::If => Ok(ForthData::DefineWord(DefineWord::If)),
                DefineWord::Then => Ok(ForthData::DefineWord(DefineWord::Then)),
                DefineWord::Else => Ok(ForthData::DefineWord(DefineWord::Else)),
            },
            ForthInstruction::BooleanOperation(boolean_operation) => {
                Ok(ForthData::BooleanOperation(boolean_operation))
            }
            ForthInstruction::LogicalOperation(logical_operation) => {
                Ok(ForthData::LogicalOperation(logical_operation))
            }
            ForthInstruction::OutputDot => Ok(ForthData::OutputDot),
            ForthInstruction::OutpuEmit => Ok(ForthData::OutpuEmit),
            ForthInstruction::OutputCR => Ok(ForthData::OutputCR),
            ForthInstruction::OutputDotQuote(string) => {
                Ok(ForthData::OutputDotQuote(String::from(string)))
            }
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

            println!("Instructions: {:?}", instructions);
            println!("Initial Stack: {:?}", stack);

            let mut i = 0;
            while i < instructions.len() {
                println!("Executing: {:?}", instructions[i]);
                println!("index: {:?}", i);
                match &instructions[i] {
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
                    ForthData::DefineWord(DefineWord::If) => {
                        let condition = stack.drop()?;
                        println!("Condition: {:?}", condition);
                        println!("index before if: {:?}", i);
                        if condition == TRUE {
                            println!("index after if: {:?}", i);
                            i += 1;
                        } else {
                            let mut skip = 1;
                            while i < instructions.len() && skip > 0 {
                                i += 1;
                                println!("Skipping if: {:?}", instructions[i]);
                                println!("Skip: {:?}", skip);
                                match &instructions[i] {
                                    ForthData::DefineWord(DefineWord::If) => skip += 1,
                                    ForthData::DefineWord(DefineWord::Then) => skip -= 1,
                                    ForthData::DefineWord(DefineWord::Else) if skip == 1 => break,
                                    _ => {}
                                }
                            }
                            println!("index after if: {:?}", i);
                        }
                    }
                    ForthData::DefineWord(DefineWord::Else) => {
                        println!("index before else: {:?}", i);
                        let mut skip = 1;
                        while i < instructions.len() && skip > 0 {
                            i += 1;
                            println!("Skipping else: {:?}", instructions[i]);
                            match &instructions[i] {
                                ForthData::DefineWord(DefineWord::If) => skip += 1,
                                ForthData::DefineWord(DefineWord::Then) => skip -= 1,
                                _ => {}
                            }
                        }
                    }
                    ForthData::DefineWord(DefineWord::Then) => {
                        i += 1;
                    }
                }
                i += 1;
            }
        }
        Ok(())
    }

    // fn conditional_check(&mut self, instructions: &[ForthData<'a>], stack: &mut Stack) {
    //     let top = stack.drop();
    //     if let Ok(value) = top {
    //         if value == TRUE {
    //             // Función que ejecutará el resto de la definición
    //             self.execute_definition(instructions, stack);
    //         } else {
    //             let else_index = self.find_index(instructions, DefineWord::Else);
    //             let then_index = self.find_index(instructions, DefineWord::Then);
    //             if let Some(index) = else_index {
    //                 // Si existe un ELSE, se salta a la siguiente definición
    //                 self.execute_definition(&instructions[index..], stack);
    //             } else if let Some(index) = then_index {
    //                 // Si no existe un ELSE, se salta a la definición de THEN
    //                 self.execute_definition(&instructions[index..], stack);
    //             }
    //         }
    //     }
    // }

    pub fn is_word_defined(&self, name: &Word) -> bool {
        self.words.contains_key(name)
    }

    pub fn get_word_definition(&self, name: &Word) -> Option<&Vec<ForthData>> {
        self.words.get(name)
    }
}

fn find_end_definition(body: &Vec<ForthInstruction>) -> Option<usize> {
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
    use crate::forth::boolean_operations::{FALSE, LogicalOperation};
    use crate::forth::intructions::ForthInstruction;
    use crate::stack::stack::Stack;
    use crate::stack::stack_errors::StackError;
    use std::io::Sink;

    #[test]
    fn can_define_new_words() {
        let mut word_manager = WordManager::new();
        // let mut stack = Stack::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![ForthData::Number(-1), ForthData::Operator("*".to_string())];

        let _ = word_manager
            .define_new_word(Word::UserDefined("NEGATE".to_string()), data)
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
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = vec![10];

        let _ = word_manager.define_new_word(Word::UserDefined("NEGATE".to_string()), word);
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
            ForthInstruction::Operator("*".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let _ = word_manager.define_new_word(Word::UserDefined("NEGATE".to_string()), word);

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

        let _ = word_manager.define_new_word(Word::UserDefined("TO-ASCCI".to_string()), word);
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
            ForthInstruction::OutputDotQuote("Hello".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = "Hello ".to_string();

        let _ = word_manager.define_new_word(Word::UserDefined("GREETING".to_string()), word);
        let _ = word_manager.execute_word(stack, &calculator, boolean_manager, writer, "GREETING");

        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_define_word_that_contains_conditionals() {
        let mut word_manger = WordManager::new();
        let word = vec![
            ForthInstruction::Number(0),
            ForthInstruction::LogicalOperation(LogicalOperation::Equal),
            ForthInstruction::DefineWord(DefineWord::If),
            ForthInstruction::OutputDotQuote("Is Zero".to_string()),
            ForthInstruction::DefineWord(DefineWord::Then),
            ForthInstruction::EndDefinition,
        ];
        let expected_result = vec![
            ForthData::Number(0),
            ForthData::LogicalOperation(LogicalOperation::Equal),
            ForthData::DefineWord(DefineWord::If),
            ForthData::OutputDotQuote("Is Zero".to_string()),
            ForthData::DefineWord(DefineWord::Then),
        ];

        let _ = word_manger.define_new_word(Word::UserDefined("is-zero?".to_string()), word);
        let result = word_manger.get_word_definition(&Word::UserDefined("is-zero?".to_string()));

        assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn can_execute_word_that_contains_conditionals() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let mut output = Vec::new();
        let writer: Option<&mut Vec<u8>> = Some(&mut output);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(0),
            ForthInstruction::LogicalOperation(LogicalOperation::Equal),
            ForthInstruction::DefineWord(DefineWord::If),
            ForthInstruction::OutputDotQuote("Is Zero".to_string()),
            ForthInstruction::DefineWord(DefineWord::Else),
            ForthInstruction::OutputDotQuote("Is Not Zero".to_string()),
            ForthInstruction::DefineWord(DefineWord::Then),
            ForthInstruction::EndDefinition,
        ];
        let expected_result = "Is Not Zero ".to_string();

        let _ = word_manager.define_new_word(Word::UserDefined("is-zero?".to_string()), word);
        let _ = stack.push(4);
        let _ = word_manager.execute_word(stack, &calculator, boolean_manager, writer, "is-zero?");
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, expected_result);
    }
}
