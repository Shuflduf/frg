use crate::ast::{Expression, expressions};

pub fn transpile(params: &[Expression]) -> String {
    let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    format!("println!({insides})")
}
