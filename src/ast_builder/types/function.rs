use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();

    let return_type = Box::new(types::parse(cursor, code));

    cursor.goto_next_sibling();
    let param_types = parameter_list(cursor, code);

    cursor.goto_parent();
    VarType::Function {
        return_type,
        param_types,
    }
}

fn parameter_list(cursor: &mut TreeCursor, code: &str) -> Vec<VarType> {
    cursor.goto_first_child();
    // skip "("
    cursor.goto_next_sibling();

    let mut param_types = vec![];
    loop {
        match cursor.node().kind() {
            ")" => break,
            "," => continue,
            "type" => param_types.push(types::parse(cursor, code)),
            _ => panic!("unexpected token in param types"),
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    cursor.goto_parent();
    param_types
}
