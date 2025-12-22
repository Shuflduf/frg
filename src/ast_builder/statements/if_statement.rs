use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> IfStatement {
    cursor.goto_first_child();
    // skip "if"
    cursor.goto_next_sibling();

    let condition = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let body = parse_block(cursor, code);

    if cursor.goto_next_sibling() {
        println!("IF {:?}", cursor.node());
    }

    cursor.goto_parent();
    IfStatement {
        condition,
        body,
        else_if_body: vec![],
        else_body: None,
    }
    // todo!()
}
