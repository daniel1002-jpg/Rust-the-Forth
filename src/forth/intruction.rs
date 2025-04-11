use crate::stack::stack_operations::StackOperation;

use super::{
    boolean_operations::{BooleanOperation, LogicalOperation},
    definition_type::DefinitionType,
};

/// Represents the different types of instructions that can be executed in the Forth interpreter
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
