use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    variable: &Expr<'ast>,
    op: &AssignmentOperator,
    expr: &Expr<'ast>,
) -> Result<(), Diagnostic> {
    let value = super::eval(state, expr)?;
    // let variable_entry = get_variable(state, variable)?;
    let target = resolve_lvalue(state, variable)?;
    let map_op_err = |e: OpError| Diagnostic::basic(e.to_string(), expr.span);
    if !value_matches_type(&target.vtype, &value) {
        let found = value.type_name();
        let expected = target.vtype_name;
        let expected_inner = target.vtype_name.inner;
        return Err(Diagnostic::new(format!(
            "cannot assign a `{found}` to a variable of type `{expected_inner}`"
        ))
        .with_label(format!("declared as `{expected_inner}`"), expected.span)
        .with_label(format!("this is `{found}`"), expr.span));
    }
    // bullshittery
    let ass = |ass_func: fn(
                    Value<'ast>,
                    Value<'ast>,
                ) -> Result<Value<'ast>, OpError>|
                 -> Result<Value<'ast>, Diagnostic> {
                     let current = target.slot.borrow().value.clone();
                    ass_func(current, value.clone()).map_err(map_op_err)
                };
    let new_value = match op {
        AssignmentOperator::Equals => value,
        AssignmentOperator::PlusEquals => ass(Value::add)?,
        AssignmentOperator::MinusEquals => ass(Value::sub)?,
        AssignmentOperator::TimesEquals => ass(Value::mul)?,
        AssignmentOperator::DivideEquals => ass(Value::div)?,
        AssignmentOperator::ModulusEquals => todo!(),
    };
    target.slot.borrow_mut().value = new_value;

    Ok(())
}
