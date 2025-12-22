use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Literal {
    // cursor.goto_first_child();

    let raw_variable = &code[cursor.node().byte_range()];
    let literal_kind = cursor.node().kind();

    // println!("{}", );

    // cursor.goto_parent();
    // todo!()
    match literal_kind {
        "int_literal" => Literal::Int(raw_variable.parse().unwrap()),
        "float_literal" => Literal::Float(raw_variable.parse().unwrap()),
        "string_literal" => Literal::Str(raw_variable[1..raw_variable.len() - 1].to_string()),
        "bool_literal" => Literal::Bool(raw_variable[1..raw_variable.len() - 1] == *"true"),
        _ => todo!(),
    }
}

pub fn transpile(lit: &Literal) -> String {
    match lit {
        Literal::Int(int) => int.to_string(),
        _ => todo!("{lit:?}"),
    }
}
