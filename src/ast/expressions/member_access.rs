use super::*;

type MemberAccess = (Box<Expression>, String);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> MemberAccess {
    cursor.goto_first_child();

    let thing_being_accessed = Box::new(expressions::parse(cursor, code));

    // skip "."
    cursor.goto_next_sibling();
    cursor.goto_next_sibling();

    let property_name = code[cursor.node().byte_range()].to_string();

    cursor.goto_parent();
    (thing_being_accessed, property_name)
}

pub fn transpile(mem_acc: &MemberAccess) -> String {
    let thing_being_accessed = expressions::transpile(&mem_acc.0);
    let prop_name = &mem_acc.1;
    format!("{thing_being_accessed}.{prop_name}")
}
