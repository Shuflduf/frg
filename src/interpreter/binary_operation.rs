use super::*;
pub fn eval<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    left: &Expr<'ast>,
    op: &BinaryOperator,
    right: &Expr<'ast>,
) -> Result<Value<'ast>, Diagnostic> {
    let left = super::eval(state, left)?;
    let right = super::eval(state, right)?;
    let map_op_err = |e: OpError| Diagnostic::basic(e.to_string(), span);
    match op {
        BinaryOperator::Add => left.add(right).map_err(map_op_err),
        BinaryOperator::Subtract => left.sub(right).map_err(map_op_err),
        BinaryOperator::Multiply => left.mul(right).map_err(map_op_err),
        BinaryOperator::Divide => left.div(right).map_err(map_op_err),
        BinaryOperator::Equals => Ok(Value::Bool(left.equals(&right).map_err(map_op_err)?)),
        BinaryOperator::GreaterThan => {
            Ok(Value::Bool(left.greater(&right, ">").map_err(map_op_err)?))
        }
        BinaryOperator::LessThan => Ok(Value::Bool(
            !left.greater(&right, "<").map_err(map_op_err)?
                && !left.equals(&right).map_err(map_op_err)?,
        )),
        BinaryOperator::GreaterThanOrEqual => Ok(Value::Bool(
            left.greater(&right, ">=").map_err(map_op_err)?
                || left.equals(&right).map_err(map_op_err)?,
        )),
        BinaryOperator::LessThanOrEqual => Ok(Value::Bool(
            !left.greater(&right, "<=").map_err(map_op_err)?,
        )),
        BinaryOperator::NotEquals => Ok(Value::Bool(!left.equals(&right).map_err(map_op_err)?)),
        other => Err(Diagnostic::basic(
            format!("unsupported binary operator: {other:?}"),
            span,
        )),
    }
}
