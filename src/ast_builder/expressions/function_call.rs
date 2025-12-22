use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> FunctionCall {
    println!(
        "code check 1.1 {} {}",
        &code[cursor.node().byte_range()],
        cursor.node()
    );
    cursor.goto_first_child();
    println!("code check 1.2 {}", &code[cursor.node().byte_range()]);
    let function = Box::new(expressions::parse(cursor, code));
    println!("function call {function:?}");

    cursor.goto_next_sibling();
    cursor.goto_next_sibling();
    println!("code check 1.3 {}", &code[cursor.node().byte_range()]);
    let params = parameters(cursor, code);
    println!("PARAMS {params:?}");

    println!("code check 1.35 {}", &code[cursor.node().byte_range()]);
    cursor.goto_parent();
    println!("code check 1.4 {}", &code[cursor.node().byte_range()]);
    FunctionCall { function, params }
}

fn parameters(cursor: &mut TreeCursor, code: &str) -> Vec<Expression> {
    println!("code check 1.31 {}", &code[cursor.node().byte_range()]);
    // skip "("
    cursor.goto_next_sibling();
    println!("code check 1.32 {}", &code[cursor.node().byte_range()]);

    let mut params = vec![];
    loop {
        let node_kind = cursor.node().kind();
        // println!("{node_kind}");
        match node_kind {
            ")" => break,
            "," | "(" => (),
            _ => params.push(expressions::parse(cursor, code)),
        }

        if !cursor.goto_next_sibling() {
            break;
        }
    }

    params
}
