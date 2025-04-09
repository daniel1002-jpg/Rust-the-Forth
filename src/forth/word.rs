use std::collections::HashMap;
use std::io::Write;
// use std::rc::Rc;

use crate::calculator::operations::Calculator;
use crate::errors::Error;
use crate::forth::forth_errors::ForthError;
use crate::forth::intructions::{DefineWord, ForthData, ForthInstruction};
use crate::stack::core::Stack;
use crate::stack::stack_operations::execute_stack_operation;

use crate::forth::boolean_operations::{BooleanOperationManager, FALSE, TRUE};

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
    words: HashMap<Word, usize>,
    definitions: Vec<Box<Vec<ForthData>>>,
    execution_stack: Vec<Word>,
    nesting_level: usize,
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
            definitions: Vec::new(),
            execution_stack: Vec::new(),
            nesting_level: 0,
        }
    }

    pub fn define_new_word(
        &mut self,
        name: Word,
        body: Vec<ForthInstruction>,
    ) -> Result<(), Error> {
        if let Word::UserDefined(ref name_str) = name {
            if !self.is_valid_word_name(name_str) {
                return Err(ForthError::InvalidWord.into());
            }
        }

        let end_index = find_end_definition(&body).ok_or(ForthError::InvalidWord)?;
        let word_definition = body.into_iter().take(end_index).collect::<Vec<_>>();
        let mut definition: Vec<ForthData> = Vec::new();

        for element in word_definition {
            definition.extend(self.convert_to_word_defintion(element)?);
            // for item in expanded_definition {
            //     definition.push(item);
            // }

            // definition.push(expanded_definition);
        }

        let index = self.definitions.len();
        self.definitions.push(Box::new(definition));

        // println!("Word definition: {:?}", self.definitions);

        self.words.insert(name, index);
        Ok(())
    }

    fn convert_to_word_defintion(
        &mut self,
        instruction: ForthInstruction,
    ) -> Result<Vec<ForthData>, Error> {
        let mut expanded_definition = Vec::new();
        match instruction {
            ForthInstruction::Number(number) => {
                expanded_definition.push(ForthData::Number(number));
                // Ok(ForthData::Number(number))
            }
            ForthInstruction::Operator(operator) => {
                expanded_definition.push(ForthData::Operator(operator.to_string()));
            }
            ForthInstruction::StackWord(stack_word) => {
                expanded_definition.push(ForthData::StackWord(stack_word));
            }
            ForthInstruction::DefineWord(define_word) => match define_word {
                DefineWord::Name(name) => {
                    if self.is_word_defined(&Word::UserDefined(name.to_string())) {
                        if let Some(&index) = self.words.get(&Word::UserDefined(name.to_string())) {
                            expanded_definition.push(ForthData::DefinitionIndex(index));
                        }
                    }
                }
                DefineWord::If => expanded_definition.push(ForthData::DefineWord(DefineWord::If)),
                DefineWord::Then => {
                    expanded_definition.push(ForthData::DefineWord(DefineWord::Then))
                }
                DefineWord::Else => {
                    expanded_definition.push(ForthData::DefineWord(DefineWord::Else))
                }
            },
            ForthInstruction::BooleanOperation(boolean_operation) => {
                expanded_definition.push(ForthData::BooleanOperation(boolean_operation))
            }
            ForthInstruction::LogicalOperation(logical_operation) => {
                expanded_definition.push(ForthData::LogicalOperation(logical_operation))
            }
            ForthInstruction::OutputDot => expanded_definition.push(ForthData::OutputDot),
            ForthInstruction::OutpuEmit => expanded_definition.push(ForthData::OutpuEmit),
            ForthInstruction::OutputCR => expanded_definition.push(ForthData::OutputCR),
            ForthInstruction::OutputDotQuote(string) => {
                expanded_definition.push(ForthData::OutputDotQuote(string.to_string()))
            }
            _ => {
                println!("Invalid word: {:?}", instruction);
                return Err(ForthError::InvalidWord.into());
            }
        }
        Ok(expanded_definition)
    }

    pub fn execute_word<W: Write>(
        &mut self,
        stack: &mut Stack,
        calculator: &Calculator,
        boolean_manager: &mut BooleanOperationManager,
        writer: &mut Option<W>,
        word_name: &str,
    ) -> Result<(), Error> {
        self.execution_stack
            .push(Word::UserDefined(word_name.to_string()));

        while let Some(current_word) = self.execution_stack.pop() {
            // println!("Executing word: {:?}", current_word);
            let index = self
                .words
                .get(&current_word)
                .ok_or(ForthError::UnknownWord)?;
            // let instructions = self.definitions.get(*index).ok_or(ForthError::UnknownWord)?;

            // let instructions = self
            //     .get_word_definition(&current_word)
            //     .ok_or(ForthError::UnknownWord)?;

            // println!("Instructions: {:?}", &instructions);
            // println!("Initial Stack: {:?}", stack);

            self.execute_instruction(*index, 0, stack, calculator, boolean_manager, writer)?;
            // self.words.insert(current_word, instructions);
        }

        self.execution_stack.clear();
        Ok(())
    }

    fn find_index_instruction(
        &self,
        def_index: usize,
        start: usize,
        target: ForthData,
    ) -> Option<usize> {
        let instructions = self
            .definitions
            .get(def_index)
            .and_then(|def| def.get(start..))
            .unwrap_or(&[]);
        // println!("Instructions: {:?}", instructions);

        let mut nesting_level = 0;
        for (offset, instruction) in instructions.iter().enumerate() {
            match instruction {
                ForthData::DefineWord(DefineWord::If) => nesting_level += 1,
                ForthData::DefineWord(DefineWord::Then) => {
                    if nesting_level == 0 && target == ForthData::DefineWord(DefineWord::Then) {
                        return Some(start + offset);
                    }
                    nesting_level -= 1;
                }
                ForthData::DefineWord(DefineWord::Else) => {
                    if nesting_level == 0 && target == ForthData::DefineWord(DefineWord::Else) {
                        return Some(start + offset);
                    }
                }
                _ => {}
            }
        }
        None
    }

    fn execute_instruction<W: Write>(
        &mut self,
        def_index: usize,
        instruction_index: usize,
        stack: &mut Stack,
        calculator: &Calculator,
        boolean_manager: &mut BooleanOperationManager,
        writer: &mut Option<W>,
    ) -> Result<(), Error> {
        let mut i = instruction_index;
        while let Some(instruction) = self.definitions.get(def_index).and_then(|def| def.get(i)) {
            // println!("\nExecuting: {:?}", &instruction);
            // println!("Actual stack: {:?}", stack.get_stack_content());
            // println!("Execution stack: {:?}", self.execution_stack);
            // if let ForthData::DefinitionIndex(index) = instruction {
            //     println!("DefinitionIndex points to: {}", index);
            // }

            // println!("Index instruction: {:?}", i);

            match &instruction {
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
                    self.execution_stack
                        .push(Word::UserDefined(name.to_string()));
                }
                ForthData::DefinitionIndex(index) => {
                    // println!("Executing DefinitionIndex: {}", index);
                    self.execute_instruction(
                        *index,
                        0,
                        stack,
                        calculator,
                        boolean_manager,
                        writer,
                    )?;
                }
                ForthData::BooleanOperation(boolean_operation) => {
                    let operand1 = stack.drop()?;
                    let operand2 = if boolean_manager.is_not(boolean_operation) {
                        None
                    } else {
                        Some(stack.drop()?)
                    };
                    let result = boolean_manager.execute_boolean_operation(
                        boolean_operation,
                        operand1,
                        operand2,
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
                        if let Some(mut w) = writer.take() {
                            // println!("{:?}", top);
                            let _ = write!(w, "{} ", top);
                            let _ = w.flush();
                            *writer = Some(w);
                        }
                    }
                }
                ForthData::OutputCR => {
                    if let Some(mut w) = writer.take() {
                        let _ = write!(w, "\n");
                        let _ = w.flush();
                        *writer = Some(w);
                    }
                }
                ForthData::OutpuEmit => {
                    if let Some(mut w) = writer.take() {
                        if let Ok(top) = stack.drop() {
                            if let Ok(ascii_char) = u8::try_from(top) {
                                // println!("{:?}", ascii_char);
                                let _ = write!(w, "{} ", ascii_char as char);
                                let _ = w.flush();
                            }
                        }
                        *writer = Some(w);
                    }
                }
                ForthData::OutputDotQuote(string) => {
                    if let Some(mut w) = writer.take() {
                        let _ = write!(w, "{} ", string);
                        let _ = w.flush();
                        *writer = Some(w);
                    }
                }
                ForthData::DefineWord(DefineWord::If) => {
                    let then_index = self.find_index_instruction(
                        def_index,
                        i + 1,
                        ForthData::DefineWord(DefineWord::Then),
                    );
                    let else_index = self.find_index_instruction(
                        def_index,
                        i + 1,
                        ForthData::DefineWord(DefineWord::Else),
                    );

                    let condition = stack.drop()?;
                    // if condition == TRUE {}
                    // println!("Condition: {:?}", condition);
                    // println!("index before if: {:?}", i);
                    // println!("Condition is: {:?}", condition == TRUE);
                    // println!("Then index exists: {:?}", then_index.is_some());

                    if let Some(then_index) = then_index {
                        // println!("Then index before conditional loop: {:?}", i + then_index);
                        self.nesting_level += 1;
                        if condition == TRUE || condition != FALSE {
                            // println!("Then index: {:?}", then_index);
                            // self.is_true_condition = true;
                            self.execute_instruction(
                                def_index,
                                i + 1,
                                stack,
                                calculator,
                                boolean_manager,
                                writer,
                            )?;
                            // println!("If loop success ended");
                            i = then_index;
                        } else {
                            if let Some(else_index) = else_index {
                                // i = else_index;
                                // println!("Else index: {:?}", else_index);
                                // self.is_true_condition = true;
                                self.execute_instruction(
                                    def_index,
                                    1 + else_index,
                                    stack,
                                    calculator,
                                    boolean_manager,
                                    writer,
                                )?;
                                // println!("If loop, else arm ended");

                                i = then_index;
                                // println!("Skipping to else: {:?}", instructions[i]);
                                // println!("index after if: {:?}", i);
                                // i += 1;
                            } else {
                                i = then_index;
                            }
                            // println!("If loop failed ended");
                            // i = then_index;
                        }
                        // i = then_index;
                        // break;
                    }
                    // println!("Index after if: {:?}", i);
                    // } else {
                    //     return Err(ForthError::InvalidWord.into());
                    // }
                }
                ForthData::DefineWord(DefineWord::Else) => {
                    if self.nesting_level > 0 {
                        // println!("True condition, skipping else");
                        // self.is_true_condition = false;
                        break;
                    }
                    // let then_index = self.find_index_instruction(
                    //     def_index,
                    //     i,
                    //     ForthData::DefineWord(DefineWord::Then));

                    // if let Ok(condition) = stack.top() {
                    //     if *condition == FALSE {

                    //     }
                    //     // println!("Skipping to else: {:?}", instructions[i]);
                    //     // println!("index after else: {:?}", i);
                    //     self.execute_instruction(
                    //         def_index,
                    //         i + 1,
                    //         stack,
                    //         calculator,
                    //         boolean_manager,
                    //         writer,
                    //     )?;
                    // } else {
                    //     if let Some(then_index) = then_index {
                    //         // println!("index after else: {:?}", i);
                    //         i += then_index;
                    //         println!("Actual index after else failed: {:?}", i);
                    //         // self.execute_instruction(
                    //         //     def_index,
                    //         //     i + then_index,
                    //         //     stack,
                    //         //     calculator,
                    //         //     boolean_manager,
                    //         //     writer,
                    //         // )?;
                    //     }
                    //     // println!("Skipping to then: {:?}", instructions[i]);
                    // }

                    // println!("index before else: {:?}", i);
                    // let mut skip = 1;
                    // while i < instructions.len() && skip > 0 {
                    //     // i += 1;
                    //     // println!("Skipping else: {:?}", instructions[i]);
                    //     match &instructions[i] {
                    //         ForthData::DefineWord(DefineWord::If) => skip += 1,
                    //         ForthData::DefineWord(DefineWord::Then) => skip -= 1,
                    //         _ => {}
                    //     }
                    // }
                }
                ForthData::DefineWord(DefineWord::Then) => {
                    if self.nesting_level > 0 {
                        self.nesting_level -= 1;
                        // println!("Ended conditional loop");
                    }

                    if self.nesting_level > 0 {
                        break;
                    }

                    // if self.is_true_condition {
                    // self.is_true_condition = false;
                    // } else {
                    //     println!("Skipping then");
                    //     // break;
                    // }
                    // break;
                    // i += 1;
                }
            }
            i += 1;
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

    pub fn get_word_definition(&self, name: &Word) -> Option<&Box<Vec<ForthData>>> {
        self.words
            .get(name)
            .and_then(|&index| self.definitions.get(index))
        // self.words.get(name)
    }

    fn is_valid_word_name(&self, name: &str) -> bool {
        if name.parse::<i16>().is_ok() {
            return false;
        }

        name.chars()
            .all(|c| c.is_alphanumeric() || c.is_ascii_punctuation())
    }
}

fn find_end_definition(body: &[ForthInstruction]) -> Option<usize> {
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
    use crate::forth::boolean_operations::LogicalOperation;
    use crate::forth::intructions::ForthInstruction;
    use crate::stack::core::Stack;
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
        let expected_result = Box::new(vec![
            ForthData::Number(-1),
            ForthData::Operator("*".to_string()),
        ]);

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
        let _ = word_manager.execute_word::<Sink>(
            stack,
            &calculator,
            boolean_manager,
            &mut None,
            "NEGATE",
        );

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

        let result = word_manager.execute_word::<Sink>(
            stack,
            &calculator,
            boolean_manager,
            &mut None,
            "ABS",
        );

        assert_eq!(result, Err(ForthError::UnknownWord.into()));
    }

    #[test]
    fn can_define_word_that_generate_output() {
        let mut word_manager = WordManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::OutpuEmit,
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = Box::new(vec![ForthData::OutpuEmit]);

        let _ = word_manager.define_new_word(Word::UserDefined("TO-ASCCI".to_string()), word);
        let result = word_manager.get_word_definition(&Word::UserDefined("TO-ASCCI".to_string()));

        assert_eq!(result, Some(&expected_result));
        // assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn can_execute_word_that_generates_output() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let mut output = Vec::new();
        let mut writer: Option<&mut Vec<u8>> = Some(&mut output);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::OutputDotQuote("Hello".to_string()),
            ForthInstruction::EndDefinition, // end
        ];
        let expected_result = "Hello ".to_string();

        let _ = word_manager.define_new_word(Word::UserDefined("GREETING".to_string()), word);
        let _ =
            word_manager.execute_word(stack, &calculator, boolean_manager, &mut writer, "GREETING");

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
        let expected_result = Box::new(vec![
            ForthData::Number(0),
            ForthData::LogicalOperation(LogicalOperation::Equal),
            ForthData::DefineWord(DefineWord::If),
            ForthData::OutputDotQuote("Is Zero".to_string()),
            ForthData::DefineWord(DefineWord::Then),
        ]);

        let _ = word_manger.define_new_word(Word::UserDefined("is-zero?".to_string()), word);
        let result = word_manger.get_word_definition(&Word::UserDefined("is-zero?".to_string()));

        assert_eq!(result, Some(&expected_result));
        // assert_eq!(result, Some(&expected_result));
    }

    #[test]
    fn can_execute_word_that_contains_conditionals() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let mut output = Vec::new();
        let mut writer: Option<&mut Vec<u8>> = Some(&mut output);
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
        let _ =
            word_manager.execute_word(stack, &calculator, boolean_manager, &mut writer, "is-zero?");
        let result = String::from_utf8(output).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_non_transitive() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word_foo: Vec<ForthInstruction> =
            vec![ForthInstruction::Number(5), ForthInstruction::EndDefinition];
        let word_bar: Vec<ForthInstruction> = vec![
            ForthInstruction::DefineWord(DefineWord::Name("foo".to_string())),
            ForthInstruction::EndDefinition,
        ];
        let redefinition_foo: Vec<ForthInstruction> =
            vec![ForthInstruction::Number(6), ForthInstruction::EndDefinition];
        let expected_result = vec![5, 6];

        let _ = word_manager.define_new_word(Word::UserDefined("foo".to_string()), word_foo);
        let _ = word_manager.define_new_word(Word::UserDefined("bar".to_string()), word_bar);
        let _ =
            word_manager.define_new_word(Word::UserDefined("foo".to_string()), redefinition_foo);

        let _ = word_manager.execute_word::<Sink>(
            stack,
            &calculator,
            boolean_manager,
            &mut None,
            "bar",
        );
        let _ = word_manager.execute_word::<Sink>(
            stack,
            &calculator,
            boolean_manager,
            &mut None,
            "foo",
        );

        let result = stack.get_stack_content();

        assert_eq!(result, &expected_result);
    }

    #[test]
    fn test_if_simple() {
        let mut word_manager = WordManager::new();
        let stack: &mut Stack = &mut Stack::new(None);
        let calculator = Calculator::new();
        let boolean_manager: &mut BooleanOperationManager = &mut BooleanOperationManager::new();
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::DefineWord(DefineWord::If),
            ForthInstruction::Number(2),
            ForthInstruction::DefineWord(DefineWord::Then),
            ForthInstruction::EndDefinition,
        ];
        let expected_result = vec![2];

        let _ = word_manager.define_new_word(Word::UserDefined("f".to_string()), word);
        let _ = stack.push(TRUE);
        let mut output = Vec::new();
        let mut writer: Option<&mut Vec<u8>> = Some(&mut output);
        let _ = word_manager.execute_word(stack, &calculator, boolean_manager, &mut writer, "f");
        let result = stack.get_stack_content();

        assert_eq!(result, &expected_result);
    }
}
