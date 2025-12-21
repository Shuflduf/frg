use super::*;

mod reference;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();

    let type_name = cursor.node().kind();
    println!("{}", &code[cursor.node().byte_range()]);
    let var_type = match type_name {
        "int" => VarType::Int,
        "reference_type" => reference::parse(cursor, code),
        _ => todo!(),
    };

    cursor.goto_parent();
    // todo!()
    var_type
}
