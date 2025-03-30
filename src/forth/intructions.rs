use crate::stack::stack_operations::StackOperation;

use super::boolean_operations::{BooleanOperation, LogicalOperation};

#[derive(Debug, PartialEq)]
pub enum ForthInstruction<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackOperation),
    StartDefinition,
    EndDefinition,
    DefineWord(DefineWord),
    BooleanOperation(&'a BooleanOperation),
    LogicalOperation(&'a LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum ForthData<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackOperation),
    DefineWord(DefineWord),
    BooleanOperation(&'a BooleanOperation),
    LogicalOperation(&'a LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(&'a str),
}

#[derive(Debug, PartialEq)]
pub enum DefineWord {
    Name(String),
    If,
    Else,
    Then,
}
