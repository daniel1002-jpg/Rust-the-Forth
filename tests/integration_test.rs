use rust_forth::{
    Forth, Instruction,
    forth::{
        boolean_operations::{FORTH_TRUE, GREATER_THAN, LESS_THAN},
        definition_type::{DefinitionType, ELSE, IF, THEN},
        forth_errors::ForthError,
        word::WordType,
        word_data::WordData,
    },
    stack::stack_operations::DROP,
};
use std::io::Sink;

#[test]
fn can_define_new_word_that_use_boolean_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let definition = vec![
        Instruction::start_definition(),
        Instruction::definition_type(DefinitionType::name("IS-POSITIVE".to_string())),
        Instruction::number(0),
        Instruction::logical_operation(GREATER_THAN),
        Instruction::end_definition(),
    ];
    let expected_result = vec![
        WordData::number(0),
        WordData::logical_operation(GREATER_THAN),
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
        Instruction::start_definition(),
        Instruction::definition_type(DefinitionType::name("DOUBLE".to_string())),
        Instruction::number(2),
        Instruction::operator("*".to_string()),
        Instruction::end_definition(),
    ];
    let data = vec![
        Instruction::number(5),
        Instruction::definition_type(DefinitionType::name("DOUBLE".to_string())),
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
        Instruction::start_definition(),
        Instruction::definition_type(DefinitionType::name("DOUBLE".to_string())),
        Instruction::number(2),
        Instruction::operator("*".to_string()),
        Instruction::end_definition(),
    ];
    let _ = forth.process_instructions(double_defintion);

    let quadruple_definition = vec![
        Instruction::start_definition(),
        Instruction::definition_type(DefinitionType::name("QUADRUPLE".to_string())),
        Instruction::definition_type(DefinitionType::name("DOUBLE".to_string())),
        Instruction::definition_type(DefinitionType::name("DOUBLE".to_string())),
        Instruction::end_definition(),
    ];

    let _ = forth.process_instructions(quadruple_definition);

    let instruction = vec![
        Instruction::number(2),
        Instruction::definition_type(DefinitionType::name("QUADRUPLE".to_string())),
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
        Instruction::number(5),
        Instruction::number(3),
        Instruction::operator("+".to_string()),
        Instruction::number(2),
        Instruction::operator("*".to_string()),
    ];
    let expected_result = Ok(&16);

    let _ = forth.process_instructions(operations);

    assert_eq!(forth.peek_stack(), expected_result);
}

#[test]
fn can_execute_logical_operations() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let operations = vec![
        Instruction::number(5),
        Instruction::number(3),
        Instruction::logical_operation(GREATER_THAN),
        Instruction::number(2),
        Instruction::logical_operation(LESS_THAN),
    ];
    let expected_result = Ok(&FORTH_TRUE);

    let _ = forth.process_instructions(operations);

    assert_eq!(forth.peek_stack(), expected_result);
}

#[test]
fn cannot_execute_unknown_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let unknown_word = vec![Instruction::definition_type(DefinitionType::name(
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
        WordData::definition_type(IF),
        WordData::definition_type(IF),
        WordData::number(1),
        WordData::definition_type(ELSE),
        WordData::number(2),
        WordData::definition_type(THEN),
        WordData::definition_type(ELSE),
        WordData::stack_word(DROP),
        WordData::number(3),
        WordData::definition_type(THEN),
    ];

    let instructions = forth.parse_instructions(input);
    let _ = forth.process_instructions(instructions);

    let result = forth.fetch_word_definition(&WordType::UserDefined("f".to_string()));

    assert_eq!(result.unwrap(), &expected_result);
}
