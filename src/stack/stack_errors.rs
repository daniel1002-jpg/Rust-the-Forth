use std::fmt;

#[derive(Debug, PartialEq)]
pub enum StackError {
    Underflow,
    Overflow,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            StackError::Underflow => write!(f, "stack-underflow"),
            StackError::Overflow => write!(f, "stack-overflow"),
        }
    }
}

impl std::error::Error for StackError {}
