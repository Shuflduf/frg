use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> FunctionCall {
    cursor.goto_first_child();
    let function = Box::new(expressions::parse(cursor, code));

    cursor.goto_next_sibling();
    let params = parameters(cursor, code);

    cursor.goto_parent();
    FunctionCall { function, params }
}

pub fn transpile(func_call: &FunctionCall) -> String {
    let func_name = expressions::transpile(&func_call.function);
    let params = expressions::transpile_list(
        &func_call
            .params
            .iter()
            .map(expressions::transpile)
            .collect(),
    );
    format!("{func_name}({params})")
}

pub fn parameters(cursor: &mut TreeCursor, code: &str) -> Vec<Expression> {
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
