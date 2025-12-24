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

    let up_to = cursor.node().kind() == "=";
    let upper_bound = if up_to {
        cursor.goto_next_sibling();
        Some((true, Box::new(expressions::parse(cursor, code))))
    } else {
        match cursor.node().kind() {
            "expression" => Some((false, Box::new(expressions::parse(cursor, code)))),
            _ => None,
        }
    };

    cursor.goto_parent();
    (lower_bound, upper_bound)
}

pub fn transpile(range_lit: &Range) -> String {
    let lower_bound = if let Some(low) = &range_lit.0 {
        expressions::transpile(low)
    } else {
        "".to_string()
    };
    let upper_bound = if let Some(up) = &range_lit.1 {
        let equal_str = if up.0 { "=" } else { "" };
        format!("{}{}", equal_str, expressions::transpile(&up.1))
    } else {
        "".to_string()
    };
    format!("{lower_bound}..{upper_bound}")
}
