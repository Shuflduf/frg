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
    let expr = expressions::parse(cursor, code);

    cursor.goto_parent();
    (field_name, expr)
}
