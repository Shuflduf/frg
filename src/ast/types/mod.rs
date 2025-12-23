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

pub fn transpile(var_type: &VarType) -> String {
    let s = match &var_type {
        VarType::Void => "()",
        VarType::Int => "i32",
        VarType::Float => "f32",
        // could be a mistake
        VarType::Str => "&'static str",
        VarType::Reference(ref_type) => &format!("&'static {}", transpile(ref_type)),
        VarType::Struct(struct_name) => struct_name,
        VarType::Function {
            return_type,
            param_types,
        } => &format!(
            "fn({}) -> {}",
            expressions::transpile_list(&param_types.iter().map(transpile).collect()),
            transpile(return_type),
        ),
        _ => todo!("{var_type:?}"),
    };
    s.to_string()
}
