use super::*;

pub fn run_macro(ctx: &mut ExecutionContext, name: &str, args: &[Expression]) -> VariableValue {
    let args = args
        .iter()
        .map(|elem| eval(ctx, elem))
        .collect::<Vec<VariableValue>>();

    match name {
        "print!" => print_macro(&args),
        "dbg!" => dbg_macro(&args),
        _ => todo!(),
    }
}

fn dbg_macro(args: &[VariableValue]) -> VariableValue {
    println!("{args:?}");

    VariableValue::Void
}

fn print_macro(args: &[VariableValue]) -> VariableValue {
    let mut output = String::new();
    let mut var_list = args.iter().peekable();
    while let Some(next_to_print) = var_list.next() {
        output.push_str(&format!("{next_to_print}"));
        if var_list.peek().is_some() {
            output.push_str(", ");
        }
    }
    println!("{output}");

    VariableValue::Void
}
