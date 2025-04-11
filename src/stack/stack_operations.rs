use crate::errors::Error;
use crate::stack::core::Stack;

/// Enum representing stack operations
/// such as duplication, dropping, swapping, etc.
/// Each operation corresponds to a specific action on the stack.
/// The operations are defined as follows:
/// - Dup: Duplicate the top element of the stack.
/// - Drop: Remove the top element of the stack.
/// - Swap: Swap the top two elements of the stack.
/// - Over: Copy the second element from the top of the stack.
/// - Rot: Rotate the top three elements of the stack.
#[derive(Debug, PartialEq)]
pub enum StackOperation {
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
}

/// Executes a stack operation on the given stack.
/// This function performs the specified operation
/// on the stack and returns a result indicating success or failure.
pub fn execute_stack_operation(stack: &mut Stack, operation: &StackOperation) -> Result<(), Error> {
    match operation {
        StackOperation::Dup => stack.dup()?,
        StackOperation::Swap => stack.swap()?,
        StackOperation::Over => stack.over()?,
        StackOperation::Rot => stack.rot()?,
        StackOperation::Drop => {
            stack.drop()?;
        }
    }
    Ok(())
}
