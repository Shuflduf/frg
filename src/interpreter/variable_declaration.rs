use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    vtype_name: &Spanned<&'ast str>,
    vtype: &VarType<'ast>,
    name: &Spanned<&'ast str>,
    value: &Option<Expr<'ast>>,
) -> Result<(), Diagnostic> {
    let mut val = Value::Undefined;
    if let Some(value) = value {
        val = super::eval(&state, &value)?;
        if !var_type_check(&vtype, &val) {
            let expected = vtype_name.inner;
            let found = val.type_name();
            return Err(Diagnostic::new(format!(
                "cannot assign a `{found}` to a variable of type `{expected}`"
            ))
            .with_label(format!("declared as `{expected}`"), vtype_name.span)
            .with_label(format!("this is `{found}`"), value.span));
        }
    }
    state.variables.insert(
        name.inner.to_string(),
        Variable {
            vtype_name: *vtype_name,
            vtype: vtype.clone(),
            value: val,
        },
    );
    Ok(())
}
