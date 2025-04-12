/// Constants for output instructions in Forth
pub const DOT: OutputInstruction = OutputInstruction::Dot;
pub const EMIT: OutputInstruction = OutputInstruction::Emit;
pub const CR: OutputInstruction = OutputInstruction::CR;

///  Enum representing the different types of output instructions in Forth
/// This includes instructions for dot, emit, carriage return (CR), and dot-quote
/// The dot instruction is used to print the top item on the stack.
/// The emit instruction is used to print a character.
/// The CR instruction is used to print a newline.
/// The dot-quote instruction is used to print a string.
#[derive(Debug, PartialEq)]
pub enum OutputInstruction {
    Dot,
    Emit,
    CR,
    DotQuote(String),
}
impl OutputInstruction {
    pub fn dot_quote(content: impl Into<String>) -> Self {
        OutputInstruction::DotQuote(content.into())
    }
}
