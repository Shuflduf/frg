use super::*;
pub fn eval<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,

    op: &UnaryOperator,
    right: &Expr<'ast>,
) -> Result<Value<'ast>, Diagnostic> {
    // let value = super::eval(state, right)?;
    // let map_op_err = |e: OpError| Diagnostic::basic(e.to_string(), span);
    match op {
        UnaryOperator::Reference => {
            if let Expression::Identifier(name) = &right.inner
                && let Some(variable) = state.variables.get(*name)
            {
                return match &variable.value {
                    Value::Type(vtype) => {
                        Ok(Value::Type(TypeValue::Reference(Box::new(vtype.clone()))))
                    }
                    _ => Ok(Value::Reference(name.to_string())),
                };
            }
            match super::eval(state, right)? {
                Value::Type(vtype) => Ok(Value::Type(TypeValue::Reference(Box::new(vtype)))),
                other => Err(Diagnostic::basic(
                    format!("cannot take a reference to `{}`", other.type_name()),
                    span,
                )),
            }
        }
        UnaryOperator::Dereference => match super::eval(state, right)? {
            Value::Reference(name) => state
                .variables
                .get(&name)
                .map(|var| var.value.clone())
                .ok_or_else(|| Diagnostic::basic("reference target not found", span)),
            other => Err(Diagnostic::basic(
                format!("cannot dereference `{}`", other.type_name()),
                span,
            )),
        },
        UnaryOperator::Negative => super::eval(state, right)?
            .neg()
            .map_err(|e| Diagnostic::basic(e.to_string(), span)),
        other => Err(Diagnostic::basic(
            format!("unsupported unary operator: {other:?}"),
            span,
        )),
    }
}
