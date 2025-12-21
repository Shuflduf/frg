use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();
    cursor.goto_next_sibling();

    let var_type = types::parse(cursor, code);

    cursor.goto_parent();

    VarType::Reference(Box::new(var_type))
}
