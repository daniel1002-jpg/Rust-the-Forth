use crate::stack::stack_operations::StackOperation;

pub enum ForthInstruction<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackOperation),
    StartDefinition,
    EndDefinition,
    DefineWord(DefineWord<'a>),
}

#[derive(Debug, PartialEq)]
pub enum ForthData<'a> {
    Number(i16),
    Operator(&'a str),
    StackWord(&'a StackOperation),
    DefineWord(DefineWord<'a>),
}

#[derive(Debug, PartialEq)]
pub enum DefineWord<'a> {
    Name(&'a str),
}
