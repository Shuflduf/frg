use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Literal {
    // cursor.goto_first_child();

    let raw_variable = &code[cursor.node().byte_range()];
    let literal_kind = cursor.node().kind();
    println!("RAW {raw_variable}");
    let literal_var = match literal_kind {
        "int_literal" => Literal::Int(raw_variable.parse().unwrap()),
        "float_literal" => Literal::Float(raw_variable.parse().unwrap()),
        "string_literal" => Literal::Str(raw_variable[1..raw_variable.len() - 1].to_string()),
        _ => todo!(),
    };

    // println!("{}", );

    // cursor.goto_parent();
    // todo!()
    literal_var
}
