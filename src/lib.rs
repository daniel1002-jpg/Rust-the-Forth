pub mod calculator;
pub mod errors;
pub mod forth;
pub mod stack;

pub use forth::forth::Forth;
pub use stack::stack::Stack;
pub use forth::boolean_operations::{BooleanOperation, LogicalOperation};
pub use forth::intructions::ForthInstruction;