use super::*;

mod function;
mod reference;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> VarType {
    cursor.goto_first_child();

    let type_name = cursor.node().kind();
    let var_type = match type_name {
        "void" => VarType::Void,
        "int" => VarType::Int,
        "str" => VarType::Str,
        "float" => VarType::Float,
        "reference_type" => reference::parse(cursor, code),
        "function_type" => function::parse(cursor, code),
        _ => todo!(),
    };

    cursor.goto_parent();
    var_type
}
