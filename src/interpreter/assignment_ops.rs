use super::*;

pub fn eval_assignment_ops(
    ctx: &mut ExecutionContext,
    name: &String,
    value: &Expression,
    op: &AssingmentOp,
) {
    let value = eval(ctx, value);
    ctx.declared_variables
        .entry(name.clone())
        .and_modify(|var| {
            // AssingmentOp::Equals => var.value = value,
            match var.borrow_mut().value {
                VariableValue::Int(ref mut int_val) => {
                    let VariableValue::Int(value) = value else {
                        panic!("{:?} and {value:?} cant be {op:?}", var.borrow().value)
                    };
                    match op {
                        AssingmentOp::PlusEquals => *int_val += value,
                        AssingmentOp::MinusEquals => *int_val -= value,
                        AssingmentOp::TimesEquals => *int_val *= value,
                        AssingmentOp::DivideEquals => *int_val /= value,
                        _ => todo!(),
                    }
                }
                _ => todo!(),
            }
        });
}
