use std::fs;

use frg::{
    ast,
    interpreter::{self, context::VariableValue},
    lexer,
};

#[test]
fn basic_addition() {
    let code_input = "return 5 + 8";
    let tokens = lexer::lex(code_input);
    let input = ast::parse(tokens);
    let VariableValue::Int(output) = interpreter::interpret(input).unwrap() else {
        panic!("")
    };
    let result = 13;
    assert_eq!(result, output)
}

#[test]
fn fibonacci() {
    let code_input = fs::read_to_string("examples/fibonacci.frg").unwrap();
    let tokens = lexer::lex(&code_input);
    let input = ast::parse(tokens);
    let VariableValue::Int(output) = interpreter::interpret(input).unwrap() else {
        panic!("")
    };
    let result = 55;
    assert_eq!(result, output)
}
