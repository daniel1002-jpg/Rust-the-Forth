use crate::{calculator::Calculator, stack::stack::Stack};

// enum ForthData {
//     Number(i16),
//     Operator(String),
// }

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

    fn push(&mut self, element: i16) -> Result<(), StackError> {
        self.stack.push(element)
    }

    fn calculate(&mut self, operator: &str) -> Result<i16, &str> {
        let operand2 = self.stack.drop()?;
        let operand1 = self.stack.drop()?;

        let result = self.calculator.calculate(operand1, operand2, operator)?;

        self.stack.push(result);

        Ok(result)
    }

    // fn calculate(&self, elements: Vec<ForthData>) -> Result<i16, &str> {
    //     let mut operands = vec![];

    //     for element in elements {
    //         match element {
    //             ForthData::Number(number) => operands.push(number),
    //             ForthData::Operator(operator) => {
    //                 let operand2 = operands.pop().unwrap();
    //                 let operand1 = operands.pop().unwrap();

    //                 let result = self.calculator.calculate(operand1, operand2, &operator)?;

    //                 operands.push(result);
    //             }
    //         }
    //     }

    //     Ok(operands.pop().unwrap())
    // }

    // fn dup(&mut self) -> Result<(), &str> {
    //     self.stack.dup()
    // }
}

mod tests {
    use crate::forth::*;

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

        // let operation: Vec<ForthData> = vec![
        //     ForthData::Number(2),
        //     ForthData::Number(4),
        //     ForthData::Operator("+".to_string()),
        // ];

        forth.push(2);
        forth.push(4);

        assert_eq!(forth.calculate("+"), Ok(6));

        // let second_operation: Vec<ForthData> = vec![
        //     ForthData::Number(2),
        //     ForthData::Number(4),
        //     ForthData::Operator("-".to_string()),
        // ];

        // let third_operation: Vec<ForthData> = vec![
        //     ForthData::Number(2),
        //     ForthData::Number(4),
        //     ForthData::Operator("*".to_string()),
        // ];

        // let fourth_operation: Vec<ForthData> = vec![
        //     ForthData::Number(2),
        //     ForthData::Number(4),
        //     ForthData::Operator("/".to_string()),
        // ];

        // assert_eq!(forth.calculate(first_operation), Ok(6));
        // assert_eq!(forth.calculate(second_operation), Ok(-2));
        // assert_eq!(forth.calculate(third_operation), Ok(8));
        // assert_eq!(forth.calculate(fourth_operation), Ok(0));
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
