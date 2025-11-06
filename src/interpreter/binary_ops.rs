use super::*;

pub fn eval_binary_ops(
    ctx: &mut ExecutionContext,
    left: Expression,
    op: BinaryOp,
    right: Expression,
) -> VariableValue {
    let left = eval(ctx, left);
    let og_left = left.clone();
    let right = eval(ctx, right);
    match left {
        VariableValue::Int(left) => {
            let VariableValue::Int(right) = right else {
                panic!("{og_left:?} and {right:?} cant be {op:?}")
            };
            match op {
                BinaryOp::Add => VariableValue::Int(left + right),
                BinaryOp::Subtract => VariableValue::Int(left - right),
                BinaryOp::Multiply => VariableValue::Int(left * right),
                BinaryOp::Divide => VariableValue::Int(left / right),
                BinaryOp::LessThan => VariableValue::Bool(left < right),
                BinaryOp::LessThanOrEqual => VariableValue::Bool(left <= right),
                BinaryOp::GreaterThan => VariableValue::Bool(left > right),
                BinaryOp::GreaterThanOrEqual => VariableValue::Bool(left >= right),
                BinaryOp::Equals => VariableValue::Bool(left == right),
            }
        }
        VariableValue::Float(left) => {
            let VariableValue::Float(right) = right else {
                panic!("{og_left:?} and {right:?} cant be {op:?}")
            };
            match op {
                BinaryOp::Add => VariableValue::Float(left + right),
                BinaryOp::Subtract => VariableValue::Float(left - right),
                BinaryOp::Multiply => VariableValue::Float(left * right),
                BinaryOp::Divide => VariableValue::Float(left / right),
                BinaryOp::LessThan => VariableValue::Bool(left < right),
                BinaryOp::LessThanOrEqual => VariableValue::Bool(left <= right),
                BinaryOp::GreaterThan => VariableValue::Bool(left > right),
                BinaryOp::GreaterThanOrEqual => VariableValue::Bool(left >= right),
                BinaryOp::Equals => VariableValue::Bool(left == right),
            }
        }
        _ => todo!(),
    }
}
