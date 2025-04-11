/// Constants for conditional definitions in Forth
/// This includes the keywords IF, ELSE, and THEN
pub const IF: DefinitionType = DefinitionType::If;
pub const ELSE: DefinitionType = DefinitionType::Else;
pub const THEN: DefinitionType = DefinitionType::Then;

/// Represents the type of a definition in Forth.
/// This includes user-defined names, conditional definitions (if, else, then)
#[derive(Debug, PartialEq)]
pub enum DefinitionType {
    Name(String),
    If,
    Else,
    Then,
}

impl DefinitionType {
    pub fn name(name: impl Into<String>) -> Self {
        DefinitionType::Name(name.into())
    }
}
