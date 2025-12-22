use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Expression {
    cursor.goto_first_child();

    // skip "return"
    cursor.goto_next_sibling();

    let expr = expressions::parse(cursor, code);

    cursor.goto_parent();
    expr
}
