use crate::interpreter::declarations::declare_variable;

use super::*;

pub fn eval_loop(
    ctx: &mut ExecutionContext,
    loop_over: &Expression,
    index_var: &str,
    body: &[Statement],
) -> Option<VariableValue> {
    let loop_count = match eval(ctx, loop_over) {
        VariableValue::Vec(loop_over_vec) => loop_over_vec.len() as i32,
        VariableValue::Int(loop_over_int) => loop_over_int,
        _ => panic!("loop over either vec or int"),
    };
    for i in 0..loop_count {
        let mut new_ctx = ctx.clone();
        declare_variable(
            &mut new_ctx,
            &VarType::Int,
            index_var,
            VariableValue::Int(i),
        );
        if let Some(early_return) = interpret_block(new_ctx, body) {
            return Some(early_return);
        }
    }
    None
}
