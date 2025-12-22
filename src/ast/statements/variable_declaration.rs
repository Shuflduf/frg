use tree_sitter::TreeCursor;

// use crate::ast_builder::{expressions, types, VariableDeclaration};
use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VariableDeclaration {
    // enter variable delcaration
    cursor.goto_first_child();
    let var_type = types::parse(cursor, code);

    cursor.goto_next_sibling();
    let identifier = code[cursor.node().byte_range()].to_string();

    // skip "="
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();

    let value = expressions::parse(cursor, code);

    cursor.goto_parent();

    VariableDeclaration {
        var_type,
        identifier,
        value,
    }
}

pub fn transpile(var_decl: &VariableDeclaration) -> String {
    let var_name = &var_decl.identifier;
    let value = expressions::transpile(&var_decl.value);
    format!("let mut {var_name} = {value};")
}
