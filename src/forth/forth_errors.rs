use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ForthError {
    InvalidWord,
    UnknownWord(String),
}

impl fmt::Display for ForthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            ForthError::InvalidWord => write!(f, "invalid-word"),
            ForthError::UnknownWord(ref word) => write!(f, "{} ?", word),
        }
    }
}

impl std::error::Error for ForthError {}
