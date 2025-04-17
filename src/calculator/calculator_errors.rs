use std::fmt;

#[derive(Debug, PartialEq)]
pub enum CalculatorError {
    DivisionByZero,
    UndefinedOperation,
}

impl fmt::Display for CalculatorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CalculatorError::DivisionByZero => write!(f, "division-by-zero"),
            CalculatorError::UndefinedOperation => write!(f, "undefined-operation"),
        }
    }
}

impl std::error::Error for CalculatorError {}
