use crate::stack::stack_operations::StackOperation;

use super::{
    boolean_operations::{BooleanOperation, LogicalOperation},
    definition_type::DefinitionType,
    output_instructions::OutputInstruction,
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
    Output(OutputInstruction),
    OutputDotQuote(String),
}

impl Instruction {
    pub fn number(value: i16) -> Self {
        Instruction::Number(value)
    }

    pub fn operator(op: impl Into<String>) -> Self {
        Instruction::Operator(op.into())
    }

    pub fn stack_word(op: StackOperation) -> Self {
        Instruction::StackWord(op)
    }

    pub fn start_definition() -> Self {
        Instruction::StartDefinition
    }

    pub fn end_definition() -> Self {
        Instruction::EndDefinition
    }

    pub fn definition_type(def: DefinitionType) -> Self {
        Instruction::DefinitionType(def)
    }

    pub fn boolean_operation(op: BooleanOperation) -> Self {
        Instruction::BooleanOperation(op)
    }

    pub fn logical_operation(op: LogicalOperation) -> Self {
        Instruction::LogicalOperation(op)
    }

    pub fn output(output: OutputInstruction) -> Self {
        Instruction::Output(output)
    }
}
