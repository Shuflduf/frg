use super::*;

type Range = (Option<Box<Expression>>, Option<(bool, Box<Expression>)>);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Range {
    println!("{}", &code[cursor.node().byte_range()]);
    cursor.goto_first_child();

    let lower_bound = match cursor.node().kind() {
        "expression" => Some(Box::new(expressions::parse(cursor, code))),
        _ => None,
    };
    if lower_bound.is_some() {
        cursor.goto_next_sibling();
    }

    // skip ".."
    cursor.goto_next_sibling();

    let up_to = match cursor.node().kind() {
        "=" => true,
        _ => false,
    };
    let upper_bound = if up_to {
        cursor.goto_next_sibling();
        Some((true, Box::new(expressions::parse(cursor, code))))
    } else {
        match cursor.node().kind() {
            "expression" => Some((false, Box::new(expressions::parse(cursor, code)))),
            _ => None,
        }
    };
    (lower_bound, upper_bound)
}

pub fn transpile(range_lit: &Range) -> String {
    todo!()
}
