use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Box<Expression> {
    cursor.goto_first_child();

    let inner = expressions::parse(cursor, code);

    cursor.goto_parent();
    Box::new(inner)
}

pub fn transpile(inner: &Expression) -> String {
    let inner_str = expressions::transpile(inner);
    format!("*{inner_str}")
}
