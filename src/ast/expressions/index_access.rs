use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> IndexAccess {
    cursor.goto_first_child();

    let expr = Box::new(expressions::parse(cursor, code));
    // println!("expr {:?}", expr);

    // skip "["
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();
    let target_index = Box::new(expressions::parse(cursor, code));

    cursor.goto_parent();
    IndexAccess { expr, target_index }
}

pub fn transpile(index_acc: &IndexAccess) -> String {
    let expr = expressions::transpile(&index_acc.expr);
    let index = expressions::transpile(&index_acc.target_index);

    format!("{expr}[{index}]")
}
