use crate::ast;

use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> FunctionLiteral {
    cursor.goto_first_child();
    let params = parameter_names(cursor, code);

    cursor.goto_next_sibling();
    cursor.goto_first_child();
    let body = parse_block(cursor, code);

    cursor.goto_parent();

    // cursor.goto_parent();
    FunctionLiteral { params, body }
}

pub fn transpile(lit: &FunctionLiteral) -> String {
    let params = expressions::transpile_list(&lit.params);

    let mut body = ast::transpile(&lit.body);
    if let Some(Statement::Expression(_)) = lit.body.last() {
        // remove ";"
        body.pop();
    }
    format!("|{params}| {{\n{body}\n}}")
}

fn parameter_names(cursor: &mut TreeCursor, code: &str) -> Vec<String> {
    cursor.goto_first_child();
    // skip "("
    cursor.goto_next_sibling();

    let mut names = vec![];
    loop {
        match cursor.node().kind() {
            ")" => break,
            "," => (),
            "identifier" => names.push(code[cursor.node().byte_range()].to_string()),
            _ => panic!("unexpected token in param list"),
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    cursor.goto_parent();

    names
}
