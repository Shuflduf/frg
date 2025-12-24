use super::*;

type Range = (Option<Box<Expression>>, Option<(bool, Box<Expression>)>);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Range {
    println!("{}", &code[cursor.node().byte_range()]);
    cursor.goto_first_child();

    let lower_bound = match cursor.node().kind() {
        "expression" => Some(expressions::parse(cursor, code)),
        _ => None,
    };
    cursor.goto_next_sibling();
    // let up_to = match cursor.node().kind() {
    //     "=" => true,
    //     "expression" => Some(expressions::parse(cursor, code)),
    //     _ => None,
    // };
    // let lower_bound = expressions::parse(cursor, code);

    // // skip ".."
    // cursor.goto_next_sibling();
    // cursor.goto_next_sibling();
    // let up_to = if cursor.node().kind() == "=" {
    //     cursor.goto_next_sibling();
    //     true
    // } else {
    //     false
    // };
    // let upper_bound = expressions::parse(cursor, code);

    // (Box::new(lower_bound), up_to, Box::new(upper_bound))
    todo!()
}

pub fn transpile(range_lit: &Range) -> String {
    todo!()
}
