use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();
    // skip "map"
    cursor.goto_next_sibling();
    // skip "("
    cursor.goto_next_sibling();

    let key_type = types::parse(cursor, code);
    let value_type = loop {
        if cursor.goto_next_sibling() {
            let node_kind = cursor.node().kind();
            let vt = match node_kind {
                "," => continue,
                ")" => panic!("value type not found"),
                _ => types::parse(cursor, code),
            };
            break vt;
        }
    };

    cursor.goto_parent();
    VarType::Map((Box::new(key_type), Box::new(value_type)))
}

pub fn transpile(inner_type: &(Box<VarType>, Box<VarType>)) -> String {
    // let inner_type_str = if let VarType::Reference(referenced_type) = inner_type {
    //     let ref_type_str = types::transpile(&referenced_type);
    //     format!("&{ref_type_str}")
    // } else {
    let key_str = types::transpile(&inner_type.0);
    let value_str = types::transpile(&inner_type.1);
    // };
    format!("std::collections::HashMap<{key_str}, {value_str}>")
}
