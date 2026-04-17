use super::{Expression, TreeCursor, expressions};

type Range = (Option<Box<Expression>>, Option<(bool, Box<Expression>)>);

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Range {
    cursor.goto_first_child();

    let lower_bound = match cursor.node().kind() {
        "expression" => Some(Box::new(expressions::parse(cursor, code))),
        _ => None,
    };
    if lower_bound.is_some() {
        cursor.goto_next_sibling();
    }

    let upper_bound: Option<(bool, Box<Expression>)>;

    match cursor.node().kind() {
        "range_all" => upper_bound = None,
        "range_to_include" => {
            cursor.goto_first_child();
            upper_bound = Some((true, Box::new(expressions::parse(cursor, code))));
            cursor.goto_parent();
        }
        "range_to" => {
            cursor.goto_first_child();
            upper_bound = Some((false, Box::new(expressions::parse(cursor, code))));
            cursor.goto_parent();
        }
        _ => unreachable!(),
    }

    cursor.goto_parent();
    (lower_bound, upper_bound)
}

pub fn transpile(range_lit: &Range) -> String {
    let lower_bound = if let Some(low) = &range_lit.0 {
        expressions::transpile(low)
    } else {
        String::new()
    };
    let upper_bound = if let Some(up) = &range_lit.1 {
        let equal_str = if up.0 { "=" } else { "" };
        format!("{}{}", equal_str, expressions::transpile(&up.1))
    } else {
        String::new()
    };
    format!("{lower_bound}..{upper_bound}")
}
