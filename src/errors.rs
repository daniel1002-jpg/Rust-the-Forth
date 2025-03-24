use crate::calculator::calculator_errors::CalculatorError;
use crate::stack::stack_errors::StackError;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Error {
    StackError(StackError),
    CalculatorError(CalculatorError),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::StackError(ref error) => write!(f, "{}", error),
            Error::CalculatorError(ref error) => write!(f, "{}", error),
        }
    }
}

impl std::error::Error for Error {}

impl From<StackError> for Error {
    fn from(error: StackError) -> Error {
        Error::StackError(error)
    }
}

impl From<CalculatorError> for Error {
    fn from(error: CalculatorError) -> Error {
        Error::CalculatorError(error)
    }
}
