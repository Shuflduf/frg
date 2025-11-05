use std::collections::HashMap;

use crate::ast::ast_nodes::*;

#[derive(Debug)]
struct VariableData {
    var_type: VarType,
    value: VariableValue,
}

#[derive(Debug)]
enum VariableValue {
    Int(i32),
}

#[derive(Debug, Default)]
struct ExecutionContext {
    declared_variables: HashMap<String, VariableData>,
}

fn interpret_block(mut ctx: ExecutionContext, ast: Vec<Statement>) {
    for statement in ast {
        match statement {
            Statement::VariableDeclaration {
                var_type,
                name,
                value,
            } => declare_variable(&mut ctx, var_type, name, value),
            _ => todo!(),
        }
    }
    dbg!(ctx);
}

pub fn interpret(ast: Vec<Statement>) {
    interpret_block(ExecutionContext::default(), ast);
}

fn declare_variable(
    ctx: &mut ExecutionContext,
    var_type: VarType,
    name: String,
    value: Expression,
) {
    let value = eval(ctx, value);
    ctx.declared_variables
        .insert(name, VariableData { value, var_type });
}

fn eval(ctx: &mut ExecutionContext, expression: Expression) -> VariableValue {
    match expression {
        Expression::Literal(literal) => match literal {
            Literal::Int(int) => VariableValue::Int(int),
            _ => todo!(),
        },
        Expression::BinaryOperation { left, op, right } => {
            let VariableValue::Int(left) = eval(ctx, *left);
            let VariableValue::Int(right) = eval(ctx, *right);

            match op {
                BinaryOp::Add => VariableValue::Int(left + right),
                _ => todo!(),
            }
        }
        _ => todo!(),
    }
}
