use crate::errors::Error;
use crate::stack::stack::Stack;

#[derive(Debug, PartialEq)]
pub enum StackOperation {
    DUP,
    DROP,
    SWAP,
    OVER,
    ROT,
}

pub fn execute_stack_operation(stack: &mut Stack, operation: &StackOperation) -> Result<(), Error> {
    match operation {
        StackOperation::DUP => stack.dup()?,
        StackOperation::SWAP => stack.swap()?,
        StackOperation::OVER => stack.over()?,
        StackOperation::ROT => stack.rot()?,
        StackOperation::DROP => { 
            stack.drop()?;
        },

    }
    Ok(())
}
