/// Represents the type of a definition in Forth.
/// This includes user-defined names, conditional definitions (if, else, then)
#[derive(Debug, PartialEq)]
pub enum DefinitionType {
    Name(String),
    If,
    Else,
    Then,
}
