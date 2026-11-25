use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    vtype_name: &Spanned<&'ast str>,
    vtype: &VarType<'ast>,
    name: &Spanned<&'ast str>,
    value: &Option<Expr<'ast>>,
) -> Result<(), Diagnostic> {
    // let mut val = Value::Undefined;
    // if let Some(value) = value {
    //     val = super::eval(&state, &value)?;
    //     if !var_type_check(&vtype, &val) {
    //         let expected = vtype_name.inner;
    //         let found = val.type_name();
    //     }
    // }
    let declared = eval_type(state, vtype)?;
    let value = match value {
        Some(expr) => {
            let value = super::eval(state, expr)?;
            if !value_matches_type(&declared, &value) {
                let expected = vtype_name.inner;
                let found = value.type_name();
                return Err(Diagnostic::new(format!(
                    "cannot assign a `{found}` to a variable of type `{expected}`"
                ))
                .with_label(format!("declared as `{expected}`"), vtype_name.span)
                .with_label(format!("this is `{found}`"), expr.span));
            }
            value
        }
        None => Value::Undefined,
    };
    state.variables.insert(
        name.inner.to_string(),
        Variable::new(*vtype_name, declared, value),
    );
    Ok(())
}
