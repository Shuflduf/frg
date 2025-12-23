use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Vec<Expression> {
    cursor.goto_first_child();
    let mut elems = vec![];

    while cursor.goto_next_sibling() {
        let node_kind = cursor.node().kind();
        elems.push(match node_kind {
            "{" | "," => continue,
            "}" => break,
            _ => expressions::parse(cursor, code),
        });
    }

    cursor.goto_parent();
    elems
}

pub fn transpile(set_lit: &[Expression]) -> String {
    let contents =
        expressions::transpile_list(&set_lit.iter().map(expressions::transpile).collect());
    format!("std::collections::HashSet::from([{contents}])")
}
