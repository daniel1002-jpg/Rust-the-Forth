// use crate::{ForthInstruction, LogicalOperation};
use super::intructions::DefineWord;
use crate::forth::boolean_operations::{BooleanOperation, LogicalOperation};
use crate::forth::intructions::ForthInstruction;
use crate::stack::stack_operations::StackOperation;

pub const LESS_THAN: LogicalOperation = LogicalOperation::LessThan;
pub const GREATER_THAN: LogicalOperation = LogicalOperation::GreaterThan;
pub const EQUAL: LogicalOperation = LogicalOperation::Equal;

pub const AND: BooleanOperation = BooleanOperation::And;
pub const OR: BooleanOperation = BooleanOperation::Or;
pub const NOT: BooleanOperation = BooleanOperation::Not;

pub const DUP: StackOperation = StackOperation::Dup;
pub const DROP: StackOperation = StackOperation::Drop;
pub const SWAP: StackOperation = StackOperation::Swap;
pub const OVER: StackOperation = StackOperation::Over;
pub const ROT: StackOperation = StackOperation::Rot;

pub struct Parser {}

impl Parser {
    pub fn new() -> Self {
        Parser {}
    }

    pub fn parse_instructions<'a>(&self, input: &'a str) -> Vec<ForthInstruction<'a>> {
        let mut instructions = Vec::new();
        let tokens = self.tokenize(input);
        dbg!(&tokens);
        for token in tokens {
            self.parse_token(&token, &mut instructions)
        }
        instructions
    }

    fn tokenize<'a>(&self, input: &'a str) -> Vec<&'a str> {
        let mut tokens = Vec::new();
        let mut in_quotes = false;
        let mut start = 0;
        let chars: Vec<char> = input.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            if chars[i] == '.' && input[i..].starts_with(".\" ") {
                if start < i {
                    tokens.push(&input[start..i]);
                }
                start = i;
                i += 2;
                in_quotes = true;

                while i < chars.len() && chars[i] != '"' {
                    i += 1;
                }

                if i < chars.len() && chars[i] == '"' {
                    tokens.push(&input[start..=i]);
                    i += 1;
                }
                in_quotes = false;
                start = i;
            } else if chars[i].is_whitespace() && !in_quotes {
                if start < i {
                    tokens.push(&input[start..i]);
                }
                start = i + 1;
                i += 1;
            } else if !in_quotes && matches!(chars[i], ':' | ';') {
                println!("i: {}", i);
                println!("chars[i]: {}", chars[i]);
                if start < i {
                    tokens.push(&input[start..i]);                
                }
                tokens.push(&input[i..=i]);
                start = i + 1;
                i += 1;
            }
             else {
                i += 1;
            }
        }
        if start < input.len() {
            tokens.push(&input[start..]);
        }
        tokens
    }

    fn parse_token<'a>(&self, token: &'a str, instructions: &mut Vec<ForthInstruction<'a>>) {
        match token {
            num if self.is_number(num) => {
                if let Ok(parsed_num) = num.parse::<i16>() {
                    instructions.push(ForthInstruction::Number(parsed_num));
                }
            }
            op if self.is_operator(op) => instructions.push(ForthInstruction::Operator(op)),
            _ if self.parse_logical_operation(token).is_some() => {
                if let Some(logical_op) = self.parse_logical_operation(token) {
                    instructions.push(logical_op);
                }
            }
            _ if self.parse_boolean_operation(token).is_some() => {
                if let Some(boolean_op) = self.parse_boolean_operation(token) {
                    instructions.push(boolean_op);
                }
            }
            _ if self.parse_stack_operation(token).is_some() => {
                if let Some(stack_op) = self.parse_stack_operation(token) {
                    instructions.push(stack_op);
                }
            }
            ":" => instructions.push(ForthInstruction::StartDefinition),
            ";" => instructions.push(ForthInstruction::EndDefinition),
            "." => instructions.push(ForthInstruction::OutputDot),
            _ if token.eq_ignore_ascii_case("emit") => {
                instructions.push(ForthInstruction::OutpuEmit)
            }
            _ if token.eq_ignore_ascii_case("cr") => instructions.push(ForthInstruction::OutputCR),
            _ if token.starts_with('.') && token.ends_with('"') => {
                let quoted_string = &token[3..token.len() - 1];
                instructions.push(ForthInstruction::OutputDotQuote(quoted_string));
            }
            _ if self.parse_word(token).is_some() => {
                if let Some(word) = self.parse_word(token) {
                    instructions.push(word);
                }
            }
            _ => (),
            // _ => instructions.push(ForthInstruction::DefineWord(DefineWord::Name(
            //     token.to_string(),
            // ))),
        }
    }

    fn is_number(&self, token: &str) -> bool {
        if token.is_empty() {
            return false;
        }
        let chars: Vec<char> = token.chars().collect();
        if chars[0] == '-' {
            return chars.len() > 1 && chars[1..].iter().all(|c| c.is_ascii_digit());
        }
        chars.iter().all(|c| c.is_ascii_digit())
    }

    fn is_operator(&self, token: &str) -> bool {
        matches!(token, "+" | "-" | "*" | "/")
    }

    fn parse_logical_operation<'a>(&self, token: &'a str) -> Option<ForthInstruction<'a>> {
        match token {
            _ if token.eq_ignore_ascii_case("<") => {
                Some(ForthInstruction::LogicalOperation(&LESS_THAN))
            }
            _ if token.eq_ignore_ascii_case(">") => {
                Some(ForthInstruction::LogicalOperation(&GREATER_THAN))
            }
            _ if token.eq_ignore_ascii_case("=") => {
                Some(ForthInstruction::LogicalOperation(&EQUAL))
            }
            _ => None,
        }
    }

    fn parse_boolean_operation<'a>(&self, token: &'a str) -> Option<ForthInstruction<'a>> {
        match token {
            _ if token.eq_ignore_ascii_case("and") => {
                Some(ForthInstruction::BooleanOperation(&AND))
            }
            _ if token.eq_ignore_ascii_case("or") => Some(ForthInstruction::BooleanOperation(&OR)),
            _ if token.eq_ignore_ascii_case("not") => {
                Some(ForthInstruction::BooleanOperation(&NOT))
            }
            _ => None,
        }
    }

    fn parse_stack_operation<'a>(&self, token: &'a str) -> Option<ForthInstruction<'a>> {
        match token {
            _ if token.eq_ignore_ascii_case("dup") => Some(ForthInstruction::StackWord(&DUP)),
            _ if token.eq_ignore_ascii_case("drop") => Some(ForthInstruction::StackWord(&DROP)),
            _ if token.eq_ignore_ascii_case("swap") => Some(ForthInstruction::StackWord(&SWAP)),
            _ if token.eq_ignore_ascii_case("over") => Some(ForthInstruction::StackWord(&OVER)),
            _ if token.eq_ignore_ascii_case("rot") => Some(ForthInstruction::StackWord(&ROT)),
            _ => None,
        }
    }

    fn parse_word<'a>(&self, token: &'a str) -> Option<ForthInstruction<'a>> {
        match token {
            _ if token.eq_ignore_ascii_case("if") => Some(ForthInstruction::DefineWord(DefineWord::If)),
            _ if token.eq_ignore_ascii_case("else") => Some(ForthInstruction::DefineWord(DefineWord::Else)),
            _ if token.eq_ignore_ascii_case("then") => Some(ForthInstruction::DefineWord(DefineWord::Then)),
            _ => Some(ForthInstruction::DefineWord(DefineWord::Name(token.to_string()))),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{forth::intructions::ForthInstruction, stack::stack_operations::StackOperation};

    #[test]
    fn can_parse_simple_instructions() {
        let parser = Parser::new();
        let input = "1 2 +";
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::Operator("+"),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_logical_instructions() {
        let parser = Parser::new();
        let input = "1 2 <";
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::LogicalOperation(&LogicalOperation::LessThan),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_boolean_instructions() {
        let parser = Parser::new();
        let input = "3 4 < 20 30 < AND";
        let expected_result = vec![
            ForthInstruction::Number(3),
            ForthInstruction::Number(4),
            ForthInstruction::LogicalOperation(&LogicalOperation::LessThan),
            ForthInstruction::Number(20),
            ForthInstruction::Number(30),
            ForthInstruction::LogicalOperation(&LogicalOperation::LessThan),
            ForthInstruction::BooleanOperation(&BooleanOperation::And),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_intruction_that_manipulate_the_stack() {
        let parser = Parser::new();
        let input = "1 2 3 DROP DUP SWAP";
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::Number(3),
            ForthInstruction::StackWord(&StackOperation::Drop),
            ForthInstruction::StackWord(&StackOperation::Dup),
            ForthInstruction::StackWord(&StackOperation::Swap),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_defined_words() {
        let parser = Parser::new();
        let input = "AWORD *WORD*";
        let expected_result = vec![
            ForthInstruction::DefineWord(DefineWord::Name("AWORD".to_string())),
            ForthInstruction::DefineWord(DefineWord::Name("*WORD*".to_string())),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definitions() {
        let parser = Parser::new();
        let input = ": NEGATE -1 * ;";
        let expected_result = vec![
            ForthInstruction::StartDefinition,
            ForthInstruction::DefineWord(DefineWord::Name("NEGATE".to_string())),
            ForthInstruction::Number(-1),
            ForthInstruction::Operator("*"),
            ForthInstruction::EndDefinition,
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_mixed_case_instructions() {
        let parser = Parser::new();
        let input = "1 2 and Dup DroP";
        let expected_result = vec![
            ForthInstruction::Number(1),
            ForthInstruction::Number(2),
            ForthInstruction::BooleanOperation(&BooleanOperation::And),
            ForthInstruction::StackWord(&StackOperation::Dup),
            ForthInstruction::StackWord(&StackOperation::Drop),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_ouput_generator_intruction() {
        let parser = Parser::new();
        let input = ". emit CR .\" Hello, World!\"";
        let expected_result = vec![
            ForthInstruction::OutputDot,
            ForthInstruction::OutpuEmit,
            ForthInstruction::OutputCR,
            ForthInstruction::OutputDotQuote("Hello, World!"),
        ];

        let result = parser.parse_instructions(input);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn can_parse_definition_with_conditionals() {
        let parser = Parser::new();
        let input = ": is-negative? 0 < IF .\" Is negative\" ELSE .\" Is positive\" then ;";
        let expected_result = vec![
            ForthInstruction::StartDefinition,
            ForthInstruction::DefineWord(DefineWord::Name("is-negative?".to_string())),
            ForthInstruction::Number(0),
            ForthInstruction::LogicalOperation(&LogicalOperation::LessThan),
            ForthInstruction::DefineWord(DefineWord::If),
            ForthInstruction::OutputDotQuote("Is negative"),
            ForthInstruction::DefineWord(DefineWord::Else),
            ForthInstruction::OutputDotQuote("Is positive"),
            ForthInstruction::DefineWord(DefineWord::Then),
            ForthInstruction::EndDefinition,
        ];
        let result = parser.parse_instructions(input);
        dbg!(&result);
        dbg!(&expected_result);

        assert_eq!(result, expected_result);
    }
}
