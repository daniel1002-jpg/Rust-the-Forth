use super::stack_errors::StackError;
use crate::errors::Error;

/// Default capacity of the stack.
pub const DEFAULT_CAPACITY: usize = 128;

/// # Stack struct
///
/// This struct represents a stack data with a fixed capacity.
///
/// ## Fields
///
/// * `capacity` - Field that represents the maximum number of elements that the stack can hold.
///             The capacity can be defined when crating the stack.     
///             If not provided, the default capacity is 128 kb.
///
/// * `size` - Field that represents the current number of elements in the stack.
///
/// * `data` - Field that holds the elements of the stack.
#[derive(Debug, PartialEq)]
pub struct Stack {
    capacity: usize,
    size: usize,
    data: Vec<i16>,
}

impl Stack {
    /// Create a new intance of the stack with a defined capacity.
    /// If not provided, the default capacity is 128 kb.
    pub fn new(capacity: Option<usize>) -> Self {
        let capacity = capacity.unwrap_or(DEFAULT_CAPACITY);
        let element_size = 2;
        let stack_capacity = capacity / element_size;

        Stack {
            capacity: stack_capacity,
            size: 0,
            data: Vec::new(),
        }
    }

    /// Get the current size of the stack.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get the capacity of the stack.
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// Check if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Push an element into the stack.
    pub fn push(&mut self, element: i16) -> Result<(), Error> {
        let is_full = self.size > self.capacity;
        if is_full {
            return Err(StackError::Overflow.into());
        }

        self.data.push(element);
        self.size += 1;
        Ok(())
    }

    /// Remove the last element from the stack.
    pub fn drop(&mut self) -> Result<i16, Error> {
        if self.is_empty() {
            return Err(StackError::Underflow.into());
        }

        let dropped = self.data.pop().ok_or(StackError::Underflow)?;
        self.size -= 1;
        Ok(dropped)
    }

    /// Get the last element from the stack, without removing it.
    pub fn top(&self) -> Result<&i16, Error> {
        match self.data.last() {
            Some(last) => Ok(last),
            None => Err(StackError::Underflow.into()),
        }
    }

    /// Duplicate the last element of the stack.
    pub fn dup(&mut self) -> Result<(), Error> {
        if self.size >= self.capacity {
            return Err(StackError::Overflow.into());
        }

        if let Ok(&top) = self.top() {
            let _ = self.push(top);
            Ok(())
        } else {
            Err(StackError::Underflow.into())
        }
    }

    /// Swap the last two elements of the stack.
    pub fn swap(&mut self) -> Result<(), Error> {
        if self.size < 2 {
            return Err(StackError::Underflow.into());
        }

        let last = self.drop()?;
        let before_last = self.drop()?;
        let _ = self.push(before_last);
        let _ = self.push(last);
        Ok(())
    }

    /// Duplicate the second element from the top of the stack.
    pub fn over(&mut self) -> Result<(), Error> {
        if self.size < 2 {
            return Err(StackError::Underflow.into());
        } else if self.size >= self.capacity {
            return Err(StackError::Overflow.into());
        }

        let last = self.drop()?;
        let before_last = *self.top()?;
        let _ = self.push(last);
        let _ = self.push(before_last);
        Ok(())
    }

    /// Rotate the top three elements of the stack.
    pub fn rot(&mut self) -> Result<(), Error> {
        if self.size < 3 {
            return Err(StackError::Underflow.into());
        }

        let mut tops = Vec::new();
        for _ in 0..2 {
            if let Ok(rotated) = self.drop() {
                tops.push(rotated);
            }
        }
        tops.reverse();

        let rotate = self.drop()?;
        for elem in tops {
            let _ = self.push(elem);
        }
        let _ = self.push(rotate);
        Ok(())
    }

    pub fn get_stack_content(&self) -> &Vec<i16> {
        &self.data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(stack.drop(), Err(StackError::Underflow.into()));
    }

    #[test]
    fn can_create_stack_with_defined_capacity() {
        // stack capacity in bytes
        let capacity = 10;
        let element_size = 2; // i16
        let stack = Stack::new(Some(capacity));
        // stack capacity expected:
        // capacity / number of bytes an element occupies
        let expected_capacty = capacity / element_size;

        assert_eq!(stack.capacity(), expected_capacty);
    }

    #[test]
    fn can_create_stack_with_default_capacity() {
        let stack = Stack::new(None);
        let element_size = 2; // i16
        let expected_capacty = DEFAULT_CAPACITY / element_size;

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

        assert_eq!(stack.push(-1), Err(StackError::Overflow.into()));
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
        assert_eq!(stack.dup(), Err(StackError::Underflow.into()));
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

        assert_eq!(stack.dup(), Err(StackError::Overflow.into()));
    }

    #[test]
    fn swapping_from_empty_stack_should_give_error() {
        let mut stack = Stack::new(None);
        assert_eq!(stack.swap(), Err(StackError::Underflow.into()));
    }

    #[test]
    fn swapping_two_elements_should_not_change_the_size() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 3];

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.swap();

        assert_eq!(stack.size(), elements.len());
    }

    #[test]
    fn can_swap_top_two_elements_in_stack() {
        let mut stack = Stack::new(None);
        let mut elements = vec![1, 3];
        let mut dropped = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.swap();
        for _ in 0..stack.size() {
            if let Ok(droped) = stack.drop() {
                dropped.push(droped);
            }
        }
        elements.reverse();

        assert_eq!(dropped, elements);
    }

    #[test]
    fn use_over_action_with_empty_stack_should_give_error() {
        let mut stack = Stack::new(None);
        assert_eq!(stack.over(), Err(StackError::Underflow.into()));
    }

    #[test]
    fn use_over_action_with_full_stack_shiuld_give_error() {
        let capacity = 2;
        let mut stack = Stack::new(Some(capacity));

        let mut element = 0;
        while stack.size() <= stack.capacity() {
            let _ = stack.push(element);
            element += 1;
        }

        assert_eq!(stack.over(), Err(StackError::Overflow.into()));
    }

    #[test]
    fn can_use_the_over_action_on_the_stack() {
        let mut stack = Stack::new(None);
        let mut elements = vec![1, 2, 3];
        let mut dropped = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.over();

        for _ in 0..stack.size() {
            if let Ok(droped) = stack.drop() {
                dropped.push(droped);
            }
        }

        let last = elements.pop().unwrap();
        let before_last = elements.last().copied().unwrap();
        elements.push(last);
        elements.push(before_last);
        elements.reverse();

        assert_eq!(dropped, elements);
    }

    #[test]
    fn can_use_the_over_action_on_the_stack_with_two_elements() {
        let mut stack = Stack::new(None);
        let mut elements = vec![1, 2];
        let mut dropped = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.over();

        for _ in 0..stack.size() {
            if let Ok(droped) = stack.drop() {
                dropped.push(droped);
            }
        }

        let last = elements.pop().unwrap();
        let before_last = elements.last().copied().unwrap();
        elements.push(last);
        elements.push(before_last);
        elements.reverse();

        assert_eq!(dropped, elements);
    }

    #[test]
    fn try_rotate_from_empty_stack_should_give_error() {
        let mut stack = Stack::new(None);
        assert_eq!(stack.rot(), Err(StackError::Underflow.into()));
    }

    #[test]
    fn can_rotate_top_three_elements_in_stack() {
        let mut stack = Stack::new(None);
        let elements = vec![1, 2, 3];
        let mut dropped = Vec::new();

        for element in &elements {
            let _ = stack.push(*element);
        }

        let _ = stack.rot();

        for _ in 0..stack.size() {
            if let Ok(droped) = stack.drop() {
                dropped.push(droped);
            }
        }

        assert_eq!(dropped, [1, 3, 2]);
    }
}
