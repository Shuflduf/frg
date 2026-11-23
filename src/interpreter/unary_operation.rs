use super::*;
pub fn eval<'ast>(
    state: &State<'ast>,
    span: SimpleSpan,

    op: &UnaryOperator,
    right: &Box<Expr<'ast>>,
) -> Result<Value<'ast>, Diagnostic> {
    let value = super::eval(state, right)?;
    let map_op_err = |e: OpError| Diagnostic::basic(e.to_string(), span);
    match op {
        UnaryOperator::Negative => value.neg().map_err(map_op_err),
        other => Err(Diagnostic::basic(
            format!("unsupported unary operator: {other:?}"),
            span,
        )),
    }
}
