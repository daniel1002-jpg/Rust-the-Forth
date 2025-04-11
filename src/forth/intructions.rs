use crate::stack::stack_operations::StackOperation;

use super::boolean_operations::{BooleanOperation, LogicalOperation};

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Number(i16),
    Operator(String),
    StackWord(StackOperation),
    StartDefinition,
    EndDefinition,
    DefinitionType(DefinitionType),
    BooleanOperation(BooleanOperation),
    LogicalOperation(LogicalOperation),
    OutputDot,
    OutpuEmit,
    OutputCR,
    OutputDotQuote(String),
}

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

#[derive(Debug, PartialEq)]
pub enum DefinitionType {
    Name(String),
    If,
    Else,
    Then,
}
