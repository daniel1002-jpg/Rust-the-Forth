use rust_forth::{
    Forth, Instruction, LogicalOperation,
    forth::{
        boolean_operations::FORTH_TRUE,
        forth_errors::ForthError,
        intructions::{DefinitionType, WordData},
        word::WordType,
    },
    stack::stack_operations::StackOperation,
};
use std::io::Sink;

#[test]
fn can_define_new_word_that_use_boolean_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let definition = vec![
        Instruction::StartDefinition,
        Instruction::DefinitionType(DefinitionType::Name("IS-POSITIVE".to_string())),
        Instruction::Number(0),
        Instruction::LogicalOperation(LogicalOperation::GreaterThan),
        Instruction::EndDefinition,
    ];
    let expected_result = vec![
        WordData::Number(0),
        WordData::LogicalOperation(LogicalOperation::GreaterThan),
    ];

    let _ = forth.process_instructions(definition);
    let result_recibed =
        forth.fetch_word_definition(&WordType::UserDefined("IS-POSITIVE".to_string()));

    assert_eq!(result_recibed, Some(&expected_result));
}

#[test]
fn can_execute_a_simple_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let definition = vec![
        Instruction::StartDefinition,
        Instruction::DefinitionType(DefinitionType::Name("DOUBLE".to_string())),
        Instruction::Number(2),
        Instruction::Operator("*".to_string()),
        Instruction::EndDefinition,
    ];
    let data = vec![
        Instruction::Number(5),
        Instruction::DefinitionType(DefinitionType::Name("DOUBLE".to_string())),
    ];
    let expected_result = Ok(&10);

    let _ = forth.process_instructions(definition);
    let _ = forth.process_instructions(data);

    assert_eq!(forth.peek_stack(), expected_result);
}

#[test]
fn can_define_nested_words_correctly() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let double_defintion = vec![
        Instruction::StartDefinition,
        Instruction::DefinitionType(DefinitionType::Name("DOUBLE".to_string())),
        Instruction::Number(2),
        Instruction::Operator("*".to_string()),
        Instruction::EndDefinition,
    ];
    let _ = forth.process_instructions(double_defintion);

    let quadruple_definition = vec![
        Instruction::StartDefinition,
        Instruction::DefinitionType(DefinitionType::Name("QUADRUPLE".to_string())),
        Instruction::DefinitionType(DefinitionType::Name("DOUBLE".to_string())),
        Instruction::DefinitionType(DefinitionType::Name("DOUBLE".to_string())),
        Instruction::EndDefinition,
    ];

    let _ = forth.process_instructions(quadruple_definition);

    let instruction = vec![
        Instruction::Number(2),
        Instruction::DefinitionType(DefinitionType::Name("QUADRUPLE".to_string())),
    ];

    let expected_result = vec![8];

    let _ = forth.process_instructions(instruction);
    let result = forth.get_stack_content();

    assert_eq!(result, &expected_result);
}

#[test]
fn can_execute_arithmetic_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let operations = vec![
        Instruction::Number(5),
        Instruction::Number(3),
        Instruction::Operator("+".to_string()),
        Instruction::Number(2),
        Instruction::Operator("*".to_string()),
    ];
    let expected_result = Ok(&16);

    let _ = forth.process_instructions(operations);

    assert_eq!(forth.peek_stack(), expected_result);
}

#[test]
fn can_execute_logical_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let operations = vec![
        Instruction::Number(5),
        Instruction::Number(3),
        Instruction::LogicalOperation(LogicalOperation::GreaterThan),
        Instruction::Number(2),
        Instruction::LogicalOperation(LogicalOperation::LessThan),
    ];
    let expected_result = Ok(&FORTH_TRUE);

    let _ = forth.process_instructions(operations);

    assert_eq!(forth.peek_stack(), expected_result);
}

#[test]
fn cannot_execute_unknown_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let unknown_word = vec![Instruction::DefinitionType(DefinitionType::Name(
        "UNKNOWN".to_string(),
    ))];

    let result = forth.process_instructions(unknown_word);

    assert_eq!(result, Err(ForthError::UnknownWord.into()));
}

#[test]
fn can_exute_a_simple_instruction() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let input = String::from("1 2 swap");
    let expected_result = vec![2, 1];

    let instructions = forth.parse_instructions(input);
    let _ = forth.process_instructions(instructions);

    assert_eq!(forth.get_stack_content(), &expected_result);
}

#[test]
fn cannot_execute_invalid_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let invalid_word = String::from(": 1 2 ;");
    let expected_result = Err(ForthError::InvalidWord.into());

    let instructions = forth.parse_instructions(invalid_word);
    println!("Instructions: {:?}", instructions);
    let result = forth.process_instructions(instructions);

    assert_eq!(result, expected_result);
}

#[test]
fn a_word_can_be_defined_on_multiple_lines() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let input = ": f
      if
        if 1 else 2 then
      else
        drop 3
      then ;"
        .to_string();
    let expected_result = vec![
        WordData::DefinitionType(DefinitionType::If),
        WordData::DefinitionType(DefinitionType::If),
        WordData::Number(1),
        WordData::DefinitionType(DefinitionType::Else),
        WordData::Number(2),
        WordData::DefinitionType(DefinitionType::Then),
        WordData::DefinitionType(DefinitionType::Else),
        WordData::StackWord(StackOperation::Drop),
        WordData::Number(3),
        WordData::DefinitionType(DefinitionType::Then),
    ];

    let instructions = forth.parse_instructions(input);
    let _ = forth.process_instructions(instructions);

    let result = forth.fetch_word_definition(&WordType::UserDefined("f".to_string()));

    assert_eq!(result.unwrap(), &expected_result);
}
