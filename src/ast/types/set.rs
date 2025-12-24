use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();
    // skip "set"
    cursor.goto_next_sibling();
    // skip "("
    cursor.goto_next_sibling();

    let inner_type = types::parse(cursor, code);

    cursor.goto_parent();
    VarType::Set(Box::new(inner_type))
}

pub fn transpile(inner_type: &VarType) -> String {
    let inner_type_str = if let VarType::Reference(referenced_type) = inner_type {
        let ref_type_str = types::transpile(referenced_type);
        format!("&{ref_type_str}")
    } else {
        types::transpile(inner_type)
    };
    format!("std::collections::HashSet<{inner_type_str}>")
}
