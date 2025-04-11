use crate::{BooleanOperation, LogicalOperation, stack::stack_operations::StackOperation};

use super::{definition_type::DefinitionType, output_instructions::OutputInstruction};

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
    Output(OutputInstruction),
    DefinitionIndex(usize),
}

impl WordData {
    pub fn number(value: i16) -> Self {
        WordData::Number(value)
    }

    pub fn operator(op: impl Into<String>) -> Self {
        WordData::Operator(op.into())
    }

    pub fn stack_word(op: StackOperation) -> Self {
        WordData::StackWord(op)
    }

    pub fn definition_type(def: DefinitionType) -> Self {
        WordData::DefinitionType(def)
    }

    pub fn boolean_operation(op: BooleanOperation) -> Self {
        WordData::BooleanOperation(op)
    }

    pub fn logical_operation(op: LogicalOperation) -> Self {
        WordData::LogicalOperation(op)
    }

    pub fn output(output: OutputInstruction) -> Self {
        WordData::Output(output)
    }
}
