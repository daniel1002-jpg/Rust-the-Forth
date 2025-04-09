use rust_forth::{
    Forth, ForthInstruction, LogicalOperation,
    forth::{
        boolean_operations::TRUE,
        forth_errors::ForthError,
        intructions::{DefineWord, ForthData},
        word::Word,
    },
    stack::stack_operations::StackOperation,
};
use std::io::Sink;

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
    let expected_result = Box::new(vec![
        ForthData::Number(0),
        ForthData::LogicalOperation(LogicalOperation::GreaterThan),
    ]);

    let _ = forth.process_data(definition);
    let result_recibed = forth.get_word_definition(&Word::UserDefined("IS-POSITIVE".to_string()));

    assert_eq!(result_recibed, Some(&expected_result));
    // assert_eq!(result_recibed, Some(&expected_result));
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
fn can_define_nested_words_correctly() {
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
        ForthInstruction::DefineWord(DefineWord::Name("DOUBLE".to_string())),
        ForthInstruction::EndDefinition,
    ];

    let _ = forth.process_data(quadruple_definition);

    let instruction = vec![
        ForthInstruction::Number(2),
        ForthInstruction::DefineWord(DefineWord::Name("QUADRUPLE".to_string())),
    ];

    let expected_result = vec![8];

    let _ = forth.process_data(instruction);
    let result = forth.get_stack_content();

    assert_eq!(result, &expected_result);

    // let expected_result = Box::new(vec![
    //     ForthData::Number(2),
    //     ForthData::Operator("*".to_string()),
    // ]);

    // assert_eq!(
    //     forth.get_word_definition(&Word::UserDefined("QUADRUPLE".to_string())),
    //     Some(&expected_result)
    // );
    // assert_eq!(
    //     forth.get_word_definition(&Word::UserDefined("QUADRUPLE".to_string())),
    //     Some(&expected_result)
    // );
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

    assert_eq!(result, Err(ForthError::UnknownWord.into()));
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

#[test]
fn cannot_execute_invalid_word() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let invalid_word = String::from(": 1 2 ;");
    let expected_result = Err(ForthError::InvalidWord.into());

    let instructions = forth.parse_instructions(invalid_word);
    println!("Instructions: {:?}", instructions);
    let result = forth.process_data(instructions);

    assert_eq!(result, expected_result);
}

#[test]
fn a_word_can_be_defined_on_multiple_lines() {
    let mut forth: Forth<Sink> = Forth::new(None, None);
    let input = String::from(
        ": f\n".to_string() + "if\n" + "if 1 else 2 then\n" + "else\n" + "drop 3\n" + "then ;\n",
    );
    let expected_result = Box::new(vec![
        ForthData::DefineWord(DefineWord::If),
        ForthData::DefineWord(DefineWord::If),
        ForthData::Number(1),
        ForthData::DefineWord(DefineWord::Else),
        ForthData::Number(2),
        ForthData::DefineWord(DefineWord::Then),
        ForthData::DefineWord(DefineWord::Else),
        ForthData::StackWord(StackOperation::Drop),
        ForthData::Number(3),
        ForthData::DefineWord(DefineWord::Then),
    ]);

    let instructions = forth.parse_instructions(input);
    let _ = forth.process_data(instructions);

    let result = forth.get_word_definition(&Word::UserDefined("f".to_string()));

    assert_eq!(result.unwrap(), &expected_result);
    // assert_eq!(result.unwrap(), &expected_result);
}
