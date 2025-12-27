use super::*;

type MapLiteral = Vec<(String, Expression)>;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> MapLiteral {
    let mut entries = vec![];
    cursor.goto_first_child();
    while cursor.goto_next_sibling() {
        let node_kind = cursor.node().kind();
        entries.push(match node_kind {
            "{" | "," => continue,
            "}" => break,
            "map_entry" => parse_map_entry(cursor, code),
            _ => todo!("{node_kind}"),
        })
    }
    cursor.goto_parent();
    entries
}

fn parse_map_entry(cursor: &mut TreeCursor, code: &str) -> (String, Expression) {
    cursor.goto_first_child();

    let field_name = code[cursor.node().byte_range()].to_string();

    // skip ":"
    cursor.goto_next_sibling();

    cursor.goto_next_sibling();
    // println!("{}", &code[cursor.node().byte_range()]);
    let expr = expressions::parse(cursor, code);

    cursor.goto_parent();
    (field_name, expr)
}

pub fn transpile(map_lit: &MapLiteral) -> String {
    let mut fields_str = String::new();
    for entry in map_lit {
        let key = &entry.0;
        let value = expressions::transpile(&entry.1);
        fields_str.push_str(&format!("({key}, {value}), "));
    }
    fields_str.pop();
    fields_str.pop();
    format!("std::collections::HashMap::from([{fields_str}])")
}

pub fn transpile_struct(map_lit: &MapLiteral) -> String {
    let mut fields_str = String::new();
    for entry in map_lit {
        let key = &entry.0;
        let value = expressions::transpile(&entry.1);
        fields_str.push_str(&format!("{key}: {value}, "));
    }
    fields_str.pop();
    fields_str.pop();
    format!("{{ {fields_str} }}")
}
