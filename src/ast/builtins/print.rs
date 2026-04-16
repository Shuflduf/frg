use crate::ast::{Expression, expressions};

pub fn transpile(params: &[Expression], new_line: bool) -> String {
    let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    if new_line {
        format!("println!({insides})")
    } else {
        format!("print!({insides})")
    }
}
