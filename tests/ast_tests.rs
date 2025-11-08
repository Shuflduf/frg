use frg::{
    ast::{self, ast_nodes::*},
    lexer,
};

#[test]
fn addition_statement() {
    let code_input = "int result = 5 + 8";
    let input = lexer::lex(code_input);
    let output = vec![Statement::VariableDeclaration {
        var_type: VarType::Int,
        name: "result".to_string(),
        value: Expression::BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(5))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(8))),
        },
    }];
    let result = ast::parse(input);
    assert_eq!(output, result)
}

#[test]
fn loop_statement() {
    let code_input = "loop numbers, i {}";
    let input = lexer::lex(code_input);
    let output = vec![Statement::Loop {
        loop_over: Expression::Identifier("numbers".to_string()),
        index_var: "i".to_string(),
        body: vec![],
    }];
    let result = ast::parse(input);
    assert_eq!(output, result)
}
