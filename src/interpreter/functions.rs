use super::*;

pub fn run_function(ctx: &mut ExecutionContext, name: &str, args: &[Expression]) -> VariableValue {
    let params: Vec<VariableValue> = args.iter().map(|exp| eval(ctx, exp)).collect();
    let target_func = ctx
        .declared_functions
        .get(name)
        .unwrap_or_else(|| panic!("function `{name}` doesnt exist"));

    let mut func_ctx = target_func.ctx.clone();
    func_ctx
        .declared_functions
        .insert(name.to_string(), target_func.clone());
    target_func
        .params
        .iter()
        .enumerate()
        .for_each(|(i, param)| {
            println!("adding var `{}` to `{}`", param.name.clone(), name);
            declarations::declare_variable(
                &mut func_ctx,
                &param.param_type.clone(),
                &param.name.clone(),
                params[i].clone(),
            );
        });

    let function_output = interpret_block(func_ctx, &target_func.ast);
    if let Some(returned_value) = function_output {
        return returned_value;
    }
    VariableValue::Void
}
