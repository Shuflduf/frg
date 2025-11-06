use super::*;

pub fn declare_function(
    ctx: &mut ExecutionContext,
    return_type: &VarType,
    name: &str,
    params: &[Parameter],
    body: &[Statement],
) {
    ctx.declared_functions.insert(
        name.to_string(),
        Rc::new(FunctionData {
            ctx: ctx.clone(),
            ast: body.to_vec(),
            params: params.to_vec(),
            return_type: return_type.clone(),
        }),
    );
}

pub fn declare_variable(
    ctx: &mut ExecutionContext,
    var_type: &VarType,
    name: &str,
    value: VariableValue,
) {
    ctx.declared_variables.insert(
        name.to_string(),
        Rc::new(RefCell::new(VariableData {
            value,
            var_type: var_type.clone(),
        })),
    );
}

pub fn declare_struct(ctx: &mut ExecutionContext, name: &str, fields: &[Parameter]) {
    let mut struct_fields = HashMap::new();
    fields.iter().for_each(|param| {
        struct_fields.insert(param.name.clone(), param.param_type.clone());
    });
    ctx.declared_structs
        .insert(name.to_string(), Rc::new(struct_fields));
}
