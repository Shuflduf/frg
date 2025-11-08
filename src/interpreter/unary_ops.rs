use super::*;

pub fn eval_unary_ops(
    ctx: &mut ExecutionContext,
    op: &UnaryOp,
    expr: &Expression,
) -> VariableValue {
    let value = eval(ctx, expr);
    match op {
        UnaryOp::Negative => match value {
            VariableValue::Int(target_int) => VariableValue::Int(-target_int),
            _ => todo!(),
        },
        _ => todo!(),
    }
}
