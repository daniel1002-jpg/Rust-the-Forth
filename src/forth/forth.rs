use crate::calculator::calculator::Calculator;
use crate::errors::Error;
use crate::stack::stack::Stack;

enum ForthData {
    Number(i16),
    Operator(String),
}

// const DUP: &str = "DUP";
// const DROP: &str = "DROP";
// const SWAP: &str = "SWAP";
// const OVER: &str = "OVER";
// const ROT: &str = "ROT";

// enum StackManipulation {
//     DUP,
//     DROP,
//     SWAP,
//     OVER,
//     ROT,
// }

pub struct Forth {
    stack: Stack,
    calculator: Calculator,
}

impl Forth {
    fn new(stack_capacity: Option<usize>) -> Self {
        Forth {
            stack: Stack::new(stack_capacity),
            calculator: Calculator,
        }
    }

    fn push(&mut self, element: i16) -> Result<(), Error> {
        self.stack.push(element)
    }

    fn calculate(&mut self, operator: &str) -> Result<i16, Error> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;

        let result = self.calculator.calculate(operand1, operand2, operator)?;

        let _ = self.stack.push(result);

        Ok(result)
    }

    fn process_data(&mut self, data: Vec<ForthData>) -> Result<(), Error> {
        for element in data {
            match element {
                ForthData::Number(number) => {
                    let _ = self.stack.push(number);
                }
                ForthData::Operator(operator) => {
                    let _ = self.calculate(&operator);
                }
            }
        }
        Ok(())
    }

    // fn dup(&mut self) -> Result<(), &str> {
    //     self.stack.dup()
    // }
}

mod tests {
    use crate::forth::forth::{Forth, ForthData};

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

    // #[test]
    // fn can_dupplicate_last_element_into_stack() {
    //     let mut forth = Forth::new(None);
    //     let elements = vec![1, 2];
    //     let duplicate_element = elements.last().cloned();

    //     for element in &elements {
    //         let _ = forth.push(*element);
    //     }

    //     let _ = forth.dup();

    //     assert_eq!(forth.stack.size(), elements.len() + 1);
    //     assert_eq!(Some(forth.stack.top().unwrap()), duplicate_element.as_ref());
    // }
}
