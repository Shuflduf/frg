use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Box<Expression> {
    cursor.goto_first_child();

    // skip "("
    cursor.goto_next_sibling();
    let inner = Box::new(expressions::parse(cursor, code));

    cursor.goto_parent();
    inner
}

pub fn transpile(inner: &Expression) -> String {
    let contents = expressions::transpile(inner);
    format!("({contents})")
}
