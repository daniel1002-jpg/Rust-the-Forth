use super::operations::StackWord;
use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::stack::stack::Stack;
use std::collections::HashMap;

enum ForthInstruction<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackWord),
    StartDefinition,
    EndDefinition,
    DefineWord(DefineWord<'a>),
}

#[derive(Debug, PartialEq)]
enum ForthData<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackWord),
    DefineWord(DefineWord<'a>),
}

#[derive(Debug, PartialEq)]
enum DefineWord<'a> {
    Name(&'a str),
    Body(Vec<&'a str>),
}

pub struct Forth<'a> {
    stack: Stack,
    calculator: Calculator,
    words: HashMap<&'a str, Vec<ForthData<'a>>>,
}

impl<'a> Forth<'a> {
    fn new(stack_capacity: Option<usize>) -> Self {
        Forth {
            stack: Stack::new(stack_capacity),
            calculator: Calculator,
            words: HashMap::new(),
        }
    }

    fn push(&mut self, element: i16) -> Result<(), Error> {
        self.stack.push(element)
    }

    fn stack_manipulate(&mut self, stack_word: &StackWord) -> Result<(), Error> {
        match stack_word {
            StackWord::DUP => self.stack.dup()?,
            StackWord::SWAP => self.stack.swap()?,
            StackWord::OVER => self.stack.over()?,
            StackWord::ROT => self.stack.rot()?,
            StackWord::DROP => {
                self.stack.drop()?;
            }
        }
        Ok(())
    }

    fn calculate(&mut self, operator: &str) -> Result<i16, Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;

        let result = self.calculator.calculate(operand1, operand2, operator)?;

        let _ = self.stack.push(result);

        Ok(result)
    }

    fn process_data(&mut self, data: Vec<ForthInstruction<'a>>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            match element {
                &ForthInstruction::Number(number) => {
                    let _ = self.stack.push(number);
                }
                ForthInstruction::Operator(operator) => {
                    let _ = self.calculate(&operator);
                }
                ForthInstruction::StackWord(stack_word) => {
                    let _ = self.stack_manipulate(stack_word);
                }
                ForthInstruction::StartDefinition => {
                    let _ = self.process_word(&data[i..]);
                    break;
                }
                _ => {}
            }
            // println!("Operación actual: {:?}", element);
            println!("{:?}", self.stack);
        }
        Ok(())
    }

    fn process_word(&mut self, data: &[ForthInstruction<'a>]) -> Result<(), Error> {
        let mut i = 0;
        for (index, element) in data.iter().enumerate() {
            i = index;
            match element {
                ForthInstruction::StartDefinition => {
                    if let ForthInstruction::DefineWord(DefineWord::Name(word_name)) = &data[i + 1]
                    {
                        let _ = self.define_new_word(word_name, &data[i + 2..]);
                        break;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn define_new_word(
        &mut self,
        word_name: &'a str,
        word_body: &[ForthInstruction<'a>],
    ) -> Result<(), Error> {
        // let mut word = Word::Name(data[1]);
        let end_index = self.find_end_definition(word_body);
        let mut word_definition = Vec::new();
        if let Some(end_index) = end_index {
            let definition = &word_body[..end_index];
            for element in definition {
                // word_definition.push(element);
                match element {
                    ForthInstruction::Number(number) => {
                        word_definition.push(ForthData::Number(*number));
                    }
                    ForthInstruction::Operator(operator) => {
                        word_definition.push(ForthData::Operator(*operator));
                    }
                    ForthInstruction::StackWord(stack_word) => {
                        word_definition.push(ForthData::StackWord(*stack_word));
                    }
                    ForthInstruction::DefineWord(define_word) => match define_word {
                        DefineWord::Name(name) => {
                            word_definition.push(ForthData::DefineWord(DefineWord::Name(name)));
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.words.insert(word_name, word_definition);
        }
        Ok(())
    }

    fn find_end_definition(&self, word_body: &[ForthInstruction<'a>]) -> Option<usize> {
        for (index, element) in word_body.iter().enumerate() {
            if let ForthInstruction::EndDefinition = element {
                return Some(index);
            }
        }
        None
    }

    fn execute_new_word(&mut self, word_name: &'a str) -> Result<(), Error> {
        let mut word_definition = self.words.get(word_name);
        if let Some(word_definition) = word_definition {
            for element in word_definition {
                match element {
                    ForthData::Number(number) => {
                        let _ = self.stack.push(*number)?;
                    }
                    ForthData::Operator(operator) => {
                        let _ = self.calculate(operator)?;
                    }
                    ForthData::StackWord(stack_word) => {
                        let _ = self.stack_manipulate(stack_word)?;
                    }
                    ForthData::DefineWord(define_word) => match define_word {
                        DefineWord::Name(name) => {
                            let _ = self.execute_new_word(name)?;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }
}

mod tests {
    use crate::forth::forth::{DefineWord, Forth, ForthData, ForthInstruction, StackWord};

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
        let _ = forth.process_data(operation);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn stack_can_be_manipulated_correctly() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(2),
            ForthInstruction::Number(4),
            ForthInstruction::StackWord(&StackWord::DUP),
            ForthInstruction::StackWord(&StackWord::ROT),
            ForthInstruction::StackWord(&StackWord::OVER),
            ForthInstruction::StackWord(&StackWord::SWAP),
            ForthInstruction::StackWord(&StackWord::DROP),
        ];
        let expected_result = vec![4, 4, 2];

        let _ = forth.process_data(data);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn can_define_new_words() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition,                        // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE")), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];

        let _ = forth.process_data(data);

        assert_eq!(forth.stack.size(), 0);
        assert_eq!(forth.words.len(), 1);
        let expected_result = vec![ForthData::Number(-1), ForthData::Operator("*")];
        let actual_definition = forth.words.get("NEGATE").unwrap();
        assert!(forth.words.contains_key("NEGATE"));
        assert_eq!(actual_definition, &expected_result);
    }

    #[test]
    fn can_execute_a_new_word_defined() {
        let mut forth = Forth::new(None);
        let word: Vec<ForthInstruction> = vec![
            ForthInstruction::StartDefinition,                        // start
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE")), // word
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition, // end
        ];

        let _ = forth.process_data(word);
        
        let data: Vec<ForthInstruction> = vec![
            ForthInstruction::Number(-10),
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE")), // word
        ];

        let _ = forth.process_data(data);

        let expected_result = vec![10];

        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }
}
