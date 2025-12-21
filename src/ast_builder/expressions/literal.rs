use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Literal {
    cursor.goto_first_child();

    let raw_variable = &code[cursor.node().byte_range()];
    let literal_var = match cursor.node().kind() {
        "number_literal" => Literal::Int(raw_variable.parse().unwrap()),
        _ => todo!(),
    };

    // println!("{}", );

    cursor.goto_parent();
    // todo!()
    literal_var
}
