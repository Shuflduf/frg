use crate::ast::{Expression, VarType, expressions};

pub fn transpile(params: &[Expression], target_type: &VarType) -> String {
    let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    match target_type {
        VarType::Int => format!("({insides}).parse::<i32>().unwrap()"),
        VarType::Str => format!("({insides}).to_string().leak()"),
        _ => todo!(),
    }
}
