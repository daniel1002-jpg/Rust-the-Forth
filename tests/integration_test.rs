use rust_forth::{
    Forth, ForthInstruction, LogicalOperation,
    forth::{
        boolean_operations::TRUE,
        forth_errors::ForthError,
        intructions::{DefineWord, ForthData},
        word::Word,
    },
};
use std::{io::Sink, rc::Rc};

#[test]
fn can_define_new_word_that_use_boolean_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let definition = vec![
        ForthInstruction::StartDefinition,
        ForthInstruction::DefineWord(DefineWord::Name("IS-POSITIVE".to_string())),
        ForthInstruction::Number(0),
        ForthInstruction::LogicalOperation(LogicalOperation::GreaterThan),
        ForthInstruction::EndDefinition,
    ];
    let expected_result = vec![
        Rc::new(ForthData::Number(0)),
        Rc::new(ForthData::LogicalOperation(LogicalOperation::GreaterThan)),
    ];

    let _ = forth.process_data(definition);
    let result_recibed = forth.get_word_definition(&Word::UserDefined("IS-POSITIVE".to_string()));

    assert_eq!(result_recibed, Some(&expected_result));
}

#[test]
fn can_execute_a_simple_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let definition = vec![
        ForthInstruction::StartDefinition,
        ForthInstruction::DefineWord(DefineWord::Name("DOUBLE".to_string())),
        ForthInstruction::Number(2),
        ForthInstruction::Operator("*".to_string()),
        ForthInstruction::EndDefinition,
    ];
    let data = vec![
        ForthInstruction::Number(5),
        ForthInstruction::DefineWord(DefineWord::Name("DOUBLE".to_string())),
    ];
    let expected_result = Ok(&10);

    let _ = forth.process_data(definition);
    let _ = forth.process_data(data);

    assert_eq!(forth.stack_top(), expected_result);
}

#[test]
fn can_define_nested_words() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let double_defintion = vec![
        ForthInstruction::StartDefinition,
        ForthInstruction::DefineWord(DefineWord::Name("DOUBLE".to_string())),
        ForthInstruction::Number(2),
        ForthInstruction::Operator("*".to_string()),
        ForthInstruction::EndDefinition,
    ];
    let _ = forth.process_data(double_defintion);

    let quadruple_definition = vec![
        ForthInstruction::StartDefinition,
        ForthInstruction::DefineWord(DefineWord::Name("QUADRUPLE".to_string())),
        ForthInstruction::DefineWord(DefineWord::Name("DOUBLE".to_string())),
        ForthInstruction::EndDefinition,
    ];
    let _ = forth.process_data(quadruple_definition);

    let expected_result = vec![
        Rc::new(ForthData::Number(2)),
        Rc::new(ForthData::Operator("*".to_string())),
    ];

    assert_eq!(
        forth.get_word_definition(&Word::UserDefined("QUADRUPLE".to_string())),
        Some(&expected_result)
    );
}

#[test]
fn can_execute_arithmetic_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let operations = vec![
        ForthInstruction::Number(5),
        ForthInstruction::Number(3),
        ForthInstruction::Operator("+".to_string()),
        ForthInstruction::Number(2),
        ForthInstruction::Operator("*".to_string()),
    ];
    let expected_result = Ok(&16);

    let _ = forth.process_data(operations);

    assert_eq!(forth.stack_top(), expected_result);
}

#[test]
fn can_execute_logical_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let operations = vec![
        ForthInstruction::Number(5),
        ForthInstruction::Number(3),
        ForthInstruction::LogicalOperation(LogicalOperation::GreaterThan),
        ForthInstruction::Number(2),
        ForthInstruction::LogicalOperation(LogicalOperation::LessThan),
    ];
    let expected_result = Ok(&TRUE);

    let _ = forth.process_data(operations);

    assert_eq!(forth.stack_top(), expected_result);
}

#[test]
fn cannot_execute_unknown_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let unknown_word = vec![ForthInstruction::DefineWord(DefineWord::Name(
        "UNKNOWN".to_string(),
    ))];

    let result = forth.process_data(unknown_word);

    assert_eq!(
        result,
        Err(ForthError::UnknownWord("UNKNOWN".to_string()).into())
    );
}

#[test]
fn can_exute_a_simple_instruction() {
    // let parser = Parser::new();
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let input = String::from("1 2 swap");
    let expected_result = vec![2, 1];

    let instructions = forth.parse_instructions(input);
    let _ = forth.process_data(instructions);

    assert_eq!(forth.get_stack_content(), &expected_result);
}