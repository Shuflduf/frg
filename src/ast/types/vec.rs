use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();
    // skip "vec"
    cursor.goto_next_sibling();
    // skip "("
    cursor.goto_next_sibling();

    let inner_type = types::parse(cursor, code);

    cursor.goto_parent();
    VarType::Vec(Box::new(inner_type))
}

pub fn transpile(inner_type: &VarType) -> String {
    let inner_type_str = types::transpile(inner_type);
    format!("Vec<{inner_type_str}>")
}
