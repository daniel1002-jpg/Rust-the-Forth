use crate::stack::stack_operations::StackOperation;

use super::boolean_operations::{BooleanOperation, LogicalOperation};

#[derive(Debug, PartialEq)]
pub enum ForthInstruction {
    Number(i16),
    Operator(String),
    StackWord(StackOperation),
    StartDefinition,
    EndDefinition,
    DefineWord(DefineWord),
    BooleanOperation(BooleanOperation),
    LogicalOperation(LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(String),
}

#[derive(Debug, PartialEq)]
pub enum ForthData {
    Number(i16),
    Operator(String),
    StackWord(StackOperation),
    DefineWord(DefineWord),
    BooleanOperation(BooleanOperation),
    LogicalOperation(LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(String),
}

#[derive(Debug, PartialEq)]
pub enum DefineWord {
    Name(String),
    If,
    Else,
    Then,
}
