use super::*;

pub fn eval<'a, 'ast>(
    state: &mut State<'ast>,
    condition: &Expr<'ast>,
    body: &'a Vec<Stmt<'ast>>,
    else_ifs: &'a Vec<(Expr<'ast>, Vec<Stmt<'ast>>)>,
    else_body: &'a Vec<Stmt<'ast>>,
) -> Result<(), Diagnostic>
where
    'a: 'ast,
{
    if let Ok(ran) = if_branch(state, condition, body)
        && ran
    {
        return Ok(());
    }

    for branch in else_ifs {
        if let Ok(ran) = if_branch(state, &branch.0, &branch.1)
            && ran
        {
            return Ok(());
        }
    }

    _ = run_statements(state, else_body);

    Ok(())
}

fn if_branch<'ast>(
    state: &mut State<'ast>,
    condition: &Expr<'_>,
    body: &'ast [Stmt<'_>],
) -> Result<bool, Diagnostic> {
    let condition_result = super::eval(&state, condition)?;
    if let Value::Bool(should_go) = condition_result {
        if should_go {
            _ = run_statements(state, body)?;
            return Ok(true);
        } else {
            return Ok(false);
        }
    } else {
        let found = condition_result.type_name();
        return Err(Diagnostic::new(format!("if statement expects a `bool`"))
            .with_label(format!("this is `{found}`"), condition.span));
    }
}
