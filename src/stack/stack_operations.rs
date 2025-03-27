use crate::errors::Error;
use crate::stack::stack::Stack;

#[derive(Debug, PartialEq)]
pub enum StackOperation {
    Dup,
    Drop,
    Swap,
    Over,
    Rot,
}

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
