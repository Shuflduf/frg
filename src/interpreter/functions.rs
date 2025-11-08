use super::*;

pub fn run_function(ctx: &mut ExecutionContext, name: &str, args: &[Expression]) -> VariableValue {
    let params: Vec<VariableValue> = args.iter().map(|exp| eval(ctx, exp)).collect();
    let is_macro = name.ends_with("!");
    if is_macro {
        return macros::run_macro(ctx, name, args);
    }
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
            // println!("adding var `{}` to `{}`", param.name.clone(), name);
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

fn conversion_error(value: &VariableValue, to: &VarType, err: impl std::error::Error) -> ! {
    panic!("Failed to parse {value:?} into {to:?} ({err})")
}

pub fn convert(ctx: &mut ExecutionContext, to: &VarType, expr: &Expression) -> VariableValue {
    let value = eval(ctx, expr);
    match to {
        VarType::Int => match value {
            VariableValue::Str(ref target_str) => VariableValue::Int(
                target_str
                    .parse()
                    .unwrap_or_else(|err| conversion_error(&value, to, err)),
            ),
            VariableValue::Float(target_float) => VariableValue::Int(target_float as i32),
            _ => todo!(),
        },
        VarType::Float => match value {
            VariableValue::Str(ref target_str) => VariableValue::Float(
                target_str
                    .parse()
                    .unwrap_or_else(|err| conversion_error(&value, to, err)),
            ),
            VariableValue::Int(target_int) => VariableValue::Float(target_int as f32),
            _ => todo!(),
        },
        VarType::Str => VariableValue::Str(value.to_string()),
        _ => todo!(),
    }
    // match value {
    //     VariableValue::Str(target_str) => match to {
    //         VarType::Int => VariableValue::Int(
    //             target_str
    //                 .parse()
    //                 .expect(&format!("failed to parse {target_str} into int")),
    //         ),
    //         VarType::Float => VariableValue::Float(
    //             target_str
    //                 .parse()
    //                 .expect(&format!("failed to parse {target_str} into float")),
    //         ),

    //         _ => todo!(),
    //     },
    //     _ => todo!(),
    // }
}
