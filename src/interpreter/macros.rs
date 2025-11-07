use super::*;

pub fn run_macro(ctx: &mut ExecutionContext, name: &str, args: &[Expression]) -> VariableValue {
    let args = args
        .iter()
        .map(|elem| eval(ctx, elem))
        .collect::<Vec<VariableValue>>();
    match name {
        "print!" => print_macro(&args),
        _ => todo!(),
    }
}
fn print_macro(args: &[VariableValue]) -> VariableValue {
    println!("{args:?}");

    VariableValue::Void
}
