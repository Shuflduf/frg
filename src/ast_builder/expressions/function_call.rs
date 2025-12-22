use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> FunctionCall {
    cursor.goto_first_child();
    let function = Box::new(expressions::parse(cursor, code));

    cursor.goto_next_sibling();
    let params = parameters(cursor, code);

    cursor.goto_parent();
    FunctionCall { function, params }
}

fn parameters(cursor: &mut TreeCursor, code: &str) -> Vec<Expression> {
    cursor.goto_first_child();
    // skip "("
    cursor.goto_next_sibling();

    let mut params = vec![];
    loop {
        let node_kind = cursor.node().kind();
        // println!("{node_kind}");
        match node_kind {
            ")" => break,
            "," => (),
            _ => params.push(expressions::parse(cursor, code)),
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    cursor.goto_parent();
    params
}
