use super::*;

pub fn eval<'ast>(
    state: &mut State<'ast>,
    condition: &Expr<'ast>,
    body: &[Stmt<'ast>],
    else_ifs: &[(Expr<'ast>, Vec<Stmt<'ast>>)],
    else_body: &[Stmt<'ast>],
) -> Result<Option<Value<'ast>>, Diagnostic> {
    if condition_true(state, condition)? {
        return super::exec_statements(state, body);
    }

    for (condition, body) in else_ifs {
        if condition_true(state, condition)? {
            return super::exec_statements(state, body);
        }
    }

    super::exec_statements(state, else_body)
}

fn condition_true<'ast>(state: &mut State<'ast>, condition: &Expr<'_>) -> Result<bool, Diagnostic> {
    match super::eval(state, condition)? {
        Value::Bool(value) => Ok(value),
        other => Err(Diagnostic::new("if statement expects a `bool`")
            .with_label(format!("this is `{}`", other.type_name()), condition.span)),
    }
}
