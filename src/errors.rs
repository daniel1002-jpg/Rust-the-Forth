use std::fmt;
use crate::stack::stack_errors::StackError;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackError(StackError),
    // CalculatorError(CalculatorError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::StackError(ref error) => write!(f, "Stack error: {}", error),
            // Error::CalculatorError(ref error) => write!(f, "Calculator error: {}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<StackError> for Error {
    fn from(error: StackError) -> Error {
        Error::StackError(error)
    }
}