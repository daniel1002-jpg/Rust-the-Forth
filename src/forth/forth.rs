use super::operations::{StackWord};
use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::stack::stack::Stack;
use std::collections::HashMap;
use std::ops::Deref;

enum ForthData<'a> {
    Number(i16),
    Operator(String),
    StackWord(StackWord),
    // Define(Define),
    StartDefinition,
    EndDefinition,
    DefineWord(DefineWord<'a>),
}

enum DefineWord<'a> {
    Name(&'a str),
    Body(Vec<&'a str>),
}

pub struct Forth<'a> {
    stack: Stack,
    calculator: Calculator,
    words: HashMap<&'a str, Vec<ForthData<'a>>>,
}

impl<'a> Forth<'_> {
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

    fn process_data(&mut self, data: Vec<ForthData>) -> Result<(), Error> {
        for (i, element) in data.iter().enumerate() {
            match element {
                ForthData::Number(number) => {
                    let _ = self.stack.push(*number);
                }
                ForthData::Operator(operator) => {
                    let _ = self.calculate(&operator);
                }
                ForthData::StackWord(stack_word) => {
                    let _ = self.stack_manipulate(stack_word);
                }
                ForthData::StartDefinition => {
                    let _ = self.process_word(&data[i..]);
                }
                _ => {}
            }
            // println!("Operación actual: {:?}", element);
            println!("{:?}", self.stack);
        }
        Ok(())
    }

    fn process_word(&mut self, data: &[ForthData]) -> Result<(), Error> {
        let mut i = 0;
        for (index, element) in data.iter().enumerate() {
            i = index;
            match element {
                ForthData::StartDefinition => {
                    if let ForthData::DefineWord(DefineWord::Name(word_name)) = data[i + 1] {
                        let _ = self.define_new_word(word_name, &data[i + 2..]);
                        break;
                    }
                }
                _ => {}
            }
        }
        
        // match prefix {
        //     Define::Start => {
        //         self.define_new_word(data)?;
        //     }
        //     Define::End => {}
        //     Define::Word(word) => {
        //         // let _ = self.process_data(data[1..].to_vec());
        //     }
        // }
        Ok(())
    }

    fn define_new_word(&mut self, word_name: &str, word_body: &[ForthData<'a>]) -> Result<(), Error> {
        // let mut word = Word::Name(data[1]);
        let end_index = self.find_end_definition(word_body);
        let mut word_definition = Vec::new();
        if let Some(end_index) = end_index {
            let definition = &word_body[..end_index];
            for element in definition {
                // word_definition.push(element);
                match element {
                    ForthData::Number(number) => {
                        word_definition.push(ForthData::Number(*number));
                    }
                    ForthData::Operator(operator) => {
                        word_definition.push(ForthData::Operator(operator.to_string()));
                    }
                    // ForthData::StackWord(stack_word) => {
                    //     word_definition.push(ForthData::StackWord(*stack_word));
                    // }
                    _ => {}
                }
            }
            self.words.insert(word_name, word_definition);
        }

        // for element in data {
        //     match element {
        //         ForthData::Define(Define::End) => {
        //             break;
        //         }
        //         ForthData::Define(Define::Word(word_name)) => {
        //             // self.words.insert(word_name, None);
        //             // word = *element;
        //         }
        //         _ => {
        //             word_data.push(element);
        //         }
        //     }
        // }
        // self.words.insert(word, Word::Body(word_data));
        Ok(())
    }

    fn find_end_definition(&self, word_body: &[ForthData<'a>]) -> Option<usize> {
        for (index, element) in word_body.iter().enumerate() {
            if let ForthData::EndDefinition = element {
                return Some(index);
            }
        }
        None
    }
}

mod tests {
    use crate::forth::{
        forth::{DefineWord, Forth, ForthData, StackWord},
        // operations::Define,
    };

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
        let operation: Vec<ForthData> = vec![
            ForthData::Number(2),
            ForthData::Number(4),
            ForthData::Operator("+".to_string()),
            ForthData::Number(6),
            ForthData::Operator("-".to_string()),
            ForthData::Number(8),
            ForthData::Number(2),
            ForthData::Operator("*".to_string()),
            ForthData::Number(4),
            ForthData::Operator("/".to_string()),
        ];

        let expected_result = vec![0, 4];
        let _ = forth.process_data(operation);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn stack_can_be_manipulated_correctly() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthData> = vec![
            ForthData::Number(2),
            ForthData::Number(4),
            ForthData::StackWord(StackWord::DUP),
            ForthData::StackWord(StackWord::ROT),
            ForthData::StackWord(StackWord::OVER),
            ForthData::StackWord(StackWord::SWAP),
            ForthData::StackWord(StackWord::DROP),
        ];
        let expected_result = vec![4, 4, 2];

        let _ = forth.process_data(data);

        assert_eq!(forth.stack.size(), expected_result.len());
        assert_eq!(forth.stack.top(), Ok(expected_result.last().unwrap()));
    }

    #[test]
    fn can_define_new_words() {
        let mut forth = Forth::new(None);
        let data: Vec<ForthData> = vec![
            ForthData::StartDefinition,                 // start
            ForthData::DefineWord(DefineWord::Name("NEGATE")), // word
            ForthData::Number(-1),
            ForthData::Operator("*".to_string()),
            ForthData::EndDefinition, // end
        ];

        let _ = forth.process_data(data);

        assert_eq!(forth.stack.size(), 0);
        // assert_eq!(forth.words.len(), 1);
        // assert_eq!(forth.words.get("NEGATE"), Some(vec![ForthData::Number(-1), ForthData::Operator("*".to_string())]));
    }
}
