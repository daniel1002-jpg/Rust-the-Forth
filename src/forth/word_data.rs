use crate::{BooleanOperation, LogicalOperation, stack::stack_operations::StackOperation};

use super::definition_type::DefinitionType;

/// Represents the different types of data that can be processed in the Forth interpreter
/// This includes numbers, operators, stack operations, and various output operations
/// Additionally, it includes types for defining new words and logical operations
#[derive(Debug, PartialEq)]
pub enum WordData {
    Number(i16),
    Operator(String),
    StackWord(StackOperation),
    DefinitionType(DefinitionType),
    BooleanOperation(BooleanOperation),
    LogicalOperation(LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(String),
    DefinitionIndex(usize),
}
