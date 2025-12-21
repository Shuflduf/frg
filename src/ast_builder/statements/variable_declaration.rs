use tree_sitter::TreeCursor;

// use crate::ast_builder::{expressions, types, VariableDeclaration};
use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VariableDeclaration {
    // enter variable delcaration
    cursor.goto_first_child();
    let var_type = types::parse(cursor, code);

    cursor.goto_next_sibling();
    let identifier = code[cursor.node().byte_range()].to_string();

    cursor.goto_next_sibling();
    // skip "="
    cursor.goto_next_sibling();

    let value = expressions::parse(cursor, code);

    cursor.goto_parent();

    VariableDeclaration {
        var_type,
        identifier,
        value,
    }
}
