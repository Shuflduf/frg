use super::*;

pub fn run_macro(ctx: &mut ExecutionContext, name: &str, args: &[Expression]) -> VariableValue {
    match name {
        "print!" => print_macro(args),
        _ => todo!(),
    }
}
fn print_macro(args: &[Expression]) -> VariableValue {
    println!("{args:?}");

    VariableValue::Void
}
