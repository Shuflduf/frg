use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    vtype_name: &Spanned<&'ast str>,
    vtype: &VarType<'ast>,
    name: &Spanned<&'ast str>,
    value: &Option<Expr<'ast>>,
) -> Result<(), Diagnostic> {
    let declared = match vtype {
        VarType::Custom(expr) => match super::eval(state, expr)? {
            Value::Type(t) => t,
            _ => {
                let target = Spanned {
                    inner: Expression::Identifier(name.inner),
                    span: name.span,
                };
                let initializer = value.as_ref().unwrap();
                return variable_assignment::eval(
                    state,
                    &target,
                    &AssignmentOperator::Equals,
                    initializer,
                );
            }
        },
        other => super::eval_type(state, other)?,
    };
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
    state.declare(&name.inner, Variable::new(*vtype_name, declared, value));
    Ok(())
}
