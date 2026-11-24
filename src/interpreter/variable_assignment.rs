use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    variable: &Expr<'ast>,
    op: &AssignmentOperator,
    expr: &Expr<'ast>,
) -> Result<(), Diagnostic> {
    let value = super::eval(state, expr)?;
    let variable_entry = get_variable(state, variable)?;
    let map_op_err = |e: OpError| Diagnostic::basic(e.to_string(), expr.span);
    if !value_matches_type(&variable_entry.vtype, &value) {
        let found = value.type_name();
        let expected = variable_entry.vtype_name;
        let expected_inner = variable_entry.vtype_name.inner;
        return Err(Diagnostic::new(format!(
            "cannot assign a `{found}` to a variable of type `{expected_inner}`"
        ))
        .with_label(format!("declared as `{expected_inner}`"), expected.span)
        .with_label(format!("this is `{found}`"), expr.span));
    }
    // bullshittery
    let mut ass = |ass_func: fn(
                    Value<'ast>,
                    Value<'ast>,
                ) -> Result<Value<'ast>, OpError>|
                 -> Result<(), Diagnostic> {
                    variable_entry.value = ass_func(variable_entry.value.clone(), value.clone())
                        .map_err(map_op_err)?;
                    Ok(())
                };
    match op {
        AssignmentOperator::Equals => variable_entry.value = value,
        AssignmentOperator::PlusEquals => ass(Value::add)?,
        AssignmentOperator::MinusEquals => ass(Value::sub)?,
        AssignmentOperator::TimesEquals => ass(Value::mul)?,
        AssignmentOperator::DivideEquals => ass(Value::div)?,
        AssignmentOperator::ModulusEquals => todo!(),
    }

    Ok(())
}
