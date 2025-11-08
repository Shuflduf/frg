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
            VariableValue::Float(target_int) => VariableValue::Float(-target_int),
            _ => panic!("cant {op:?} on {value:?}"),
        },
        UnaryOp::Not => {
            if let VariableValue::Bool(target_bool) = value {
                VariableValue::Bool(!target_bool)
            } else {
                panic!("cant {op:?} on {value:?}")
            }
        }
        _ => todo!(),
    }
}
