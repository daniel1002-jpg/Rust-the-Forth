// const UNDERFLOW_ERROR: &str = "stack-underflow";
// const OVERFLOW_ERROR: &str = "stack-overflow";
const DEFAULT_CAPACITY: usize = 128;

#[derive(Debug, PartialEq)]
pub enum StackError {
    StackUnderflow,
    StackOverflow,
}

pub struct Stack {
    capacity: usize,
    size: usize,
    data: Vec<i16>,
}

impl Stack {
    pub fn new(capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(DEFAULT_CAPACITY);
        let stack_capacity = capacity * 1024 / 2;

        Stack {
            capacity: stack_capacity,
            size: 0,
            data: Vec::new(),
        }
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn push(&mut self, element: i16) -> Result<(), StackError> {
        let is_full = self.size > self.capacity;
        if is_full {
            return Err(StackError::StackOverflow);
        }

        self.data.push(element);
        self.size += 1;
        Ok(())
    }

    pub fn drop(&mut self) -> Result<i16, StackError> {
        if self.is_empty() {
            return Err(StackError::StackUnderflow);
        }

        let dropped = self.data.
                                                    pop().
                                                    ok_or(StackError::StackUnderflow);
        self.size -= 1;
        dropped
    }

    pub fn top(&self) -> Result<&i16, StackError> {
        match self.data.last() {
            Some(last) => Ok(last),
            None => Err(StackError::StackUnderflow),
        }
    }

    pub fn dup(&mut self) -> Result<(), StackError> {
        if self.size >= self.capacity {
            return Err(StackError::StackOverflow);
        }
        
        if let Ok(&top) = self.top() {
            let _ = self.push(top);
            Ok(())
        } else {
            Err(StackError::StackUnderflow)
        }
    }

    // pub fn swap(&mut self) -> Result<(), &str> {
    //     if self.size < 2 {
    //         return Err(UNDERFLOW_ERROR);
    //     }

    //     let last = self.drop().or_else(|_| Err(UNDERFLOW_ERROR))?;
    //     let before_last = self.drop().or_else(|_| Err(UNDERFLOW_ERROR))?;

    //     let _ = self.push(before_last);
    //     let _ = self.push(last);

    //     Ok(())
    // }

    // pub fn over(&mut self) -> Result<(), &str> {
    //     if self.size < 2 {
    //         return Err(UNDERFLOW_ERROR);
    //     }

    //     let last = self.drop().unwrap();
    //     let before_last = self.drop().unwrap();

    //     let _ = self.push(before_last);
    //     let _ = self.push(last);
    //     let _ = self.push(before_last);

    //     Ok(())
    // }
}

#[cfg(test)]
mod tests {
    use crate::stack::*;

    #[test]
    fn an_empty_stack_can_be_created_successsfully() {
        let stack = Stack::new(None);
        assert!(stack.is_empty());
    }

    #[test]
    fn a_stack_is_not_empty_after_pushing_an_element() {
        let mut stack = Stack::new(None);
        let _ = stack.push(1);

        assert!(!stack.is_empty());
    }

    #[test]
    fn can_push_elements_into_stack_correctly() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 2, -3];

        for element in &elements {
            let _ = stack.push(*element);
        }

        assert_eq!(stack.size(), elements.len());
    }

    #[test]
    fn drop_elements_of_stack_reduce_size() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 2, -3];

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.drop();
        let _ = stack.drop();

        assert_eq!(stack.size(), elements.len() - 2);
    }

    #[test]
    fn can_drop_elements_from_stack_correctly() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 2, -3];
        let mut droped_elements: Vec<Result<i16, StackError>> = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        if let Ok(dropped) = stack.drop() {
            droped_elements.push(Ok(dropped));
        }
        if let Ok(dropped) = stack.drop() {
            droped_elements.push(Ok(dropped));
        }
        
        assert_eq!(droped_elements, vec![Ok(-3), Ok(2)]);
    }

    #[test]
    fn can_drop_elements_from_stack_until_empty() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 2, -3];
        let mut droped_elements: Vec<Result<i16, &str>> = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        while !stack.is_empty() {
            if let Ok(droped) = stack.drop() {
                droped_elements.push(Ok(droped));
            }
        }

        assert_eq!(droped_elements, vec![Ok(-3), Ok(2), Ok(1)]);
        assert_eq!(stack.size(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn dropping_from_empty_stack_should_give_error() {
        let mut stack = Stack::new(None);
        assert_eq!(stack.drop(), Err(StackError::StackUnderflow));
    }

    #[test]
    fn can_create_stack_with_defined_capacity() {
        // stack capacity in bytes
        let capacity = 10;
        let stack = Stack::new(Some(capacity));
        // stack capacity expected:
        // capacity * 1024 / number of bytes an element occupies
        let expected_capacty = capacity * 1024 / 2;

        assert_eq!(stack.capacity(), expected_capacty);
    }

    #[test]
    fn can_create_stack_with_default_capacity() {
        let stack = Stack::new(None);
        let expected_capacty = DEFAULT_CAPACITY * 1024 / 2;

        assert_eq!(stack.capacity(), expected_capacty);
    }

    #[test]
    fn attempting_to_load_stack_beyond_capacity_should_give_error() {
        let capacity = 2;
        let mut stack = Stack::new(Some(capacity));

        let mut element = 0;
        while stack.size() <= stack.capacity() {
            let _ = stack.push(element);
            element += 1;
        }

        assert_eq!(stack.push(-1), Err(StackError::StackOverflow));
    }

    #[test]
    fn can_get_top_of_the_stack() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 3];
        let last = elements.last().copied();

        for element in elements {
            let _ = stack.push(element);
        }

        assert_eq!(Some(stack.top().unwrap()), last.as_ref());
    }

    #[test]
    fn can_dupplicate_last_element_into_stack() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 3];
        let last = elements.last().copied();

        for element in &elements {
            let _ = stack.push(*element);
        }

        assert_eq!(stack.dup(), Ok(()));
        assert_eq!(stack.size(), elements.len() + 1);
        assert_eq!(Some(stack.top().unwrap()), last.as_ref());
    }

    #[test]
    fn try_dupplicate_from_empty_stack_should_give_error() {
        let mut stack = Stack::new(None);
        assert_eq!(stack.dup(), Err(StackError::StackUnderflow));
    }

    #[test]
    fn try_dupplicate_from_full_stack_should_give_error() {
        let capacity = 2;
        let mut stack = Stack::new(Some(capacity));

        let mut element = 0;
        while stack.size() <= stack.capacity() {
            let _ = stack.push(element);
            element += 1;
        }

        println!("stack size: {}", stack.size());
        println!("stack capacity: {}", stack.capacity());

        assert_eq!(stack.dup(), Err(StackError::StackOverflow));
    }

    // #[test]
    // fn swapping_from_empty_stack_should_give_error() {
    //     let mut stack = Stack::new(None);
    //     assert_eq!(stack.swap(), Err(UNDERFLOW_ERROR));
    // }

    // #[test]
    // fn swapping_two_elements_should_not_change_the_size() {
    //     let mut stack = Stack::new(None);
    //     let elements = vec![1, 3];
        
    //     for element in &elements {
    //         let _ = stack.push(*element);
    //     }

    //     let _ = stack.swap();

    //     assert_eq!(stack.size(), elements.len());
    // }

    // #[test]
    // fn can_swap_top_two_elements_in_stack() {
    //     let mut stack = Stack::new(None);
    //     let mut elements = vec![1, 3];
    //     let mut dropped = Vec::new();

    //     for element in &elements {
    //         let _ = stack.push(*element);
    //     }


    //     let _ = stack.swap();
    //     for _ in 0..stack.size() {
    //         if let Ok(droped) = stack.drop() {
    //             dropped.push(droped);
    //         }
    //     }
    //     elements.reverse();

    //     assert_eq!(dropped, elements);
    // }

    // #[test]
    // fn can_use_the_over_action_on_the_stack() {
    //     let mut stack = Stack::new(None);
    //     let mut elements = vec![1, 2, 3];
    //     let mut dropped = Vec::new();

    //     for element in &elements {
    //         let _ = stack.push(*element);
    //     }

    //     let _ = stack.over();

    //     for _ in 0..stack.size() {
    //         if let Ok(droped) = stack.drop() {
    //             dropped.push(droped);
    //         }
    //     }

    //     let last = elements.last().unwrap();
    //     elements.insert(elements.len() - 1, *elements.last().unwrap());
    //     elements.push(*last);

    //     println!("stack result: {:?}", dropped);
    //     println!("expected result: {:?}", elements);

    //     assert_eq!(dropped, elements);
    // }
}
