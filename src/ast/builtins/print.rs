use crate::ast::{Expression, expressions};

pub fn transpile(params: &Vec<Expression>) -> String {
    let insides =
        expressions::transpile_list(&params.iter().map(|p| expressions::transpile(p)).collect());
    format!("println!({insides})")
}
