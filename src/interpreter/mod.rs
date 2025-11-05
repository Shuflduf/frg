use std::collections::{HashMap, HashSet};

use crate::ast::ast_nodes::*;

#[derive(Debug, Clone)]
struct VariableData {
    var_type: VarType,
    value: VariableValue,
}

#[derive(Debug, Clone)]
enum VariableValue {
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone)]
struct FunctionData {
    ctx: ExecutionContext,
    ast: Vec<Statement>,
}

#[derive(Debug, Default, Clone)]
struct ExecutionContext {
    declared_variables: HashMap<String, VariableData>,
    declared_functions: HashMap<String, FunctionData>,
    callable_functions: HashSet<String>,
    continue_to_next_if: bool,
}

fn interpret_block(mut ctx: ExecutionContext, ast: Vec<Statement>) {
    for statement in ast {
        match statement {
            Statement::VariableDeclaration {
                var_type,
                name,
                value,
            } => declare_variable(&mut ctx, var_type, name, value),
            Statement::FunctionDeclaration {
                return_type,
                name,
                params,
                body,
            } => declare_function(&mut ctx, return_type, name, params, body),
            Statement::Conditional {
                conditional_type,
                body,
            } => handle_conditionals(&mut ctx, conditional_type, body),
            _ => todo!(),
        }
    }
    dbg!(ctx);
}

pub fn interpret(ast: Vec<Statement>) {
    interpret_block(ExecutionContext::default(), ast);
}

fn handle_conditionals(
    ctx: &mut ExecutionContext,
    conditional_type: ConditionalType,
    body: Vec<Statement>,
) {
    if let VariableValue::Bool(should_run) = match conditional_type {
        ConditionalType::If(expression) => {
            ctx.continue_to_next_if = true;
            eval(ctx, expression)
        }
        ConditionalType::Elif(expression) => eval(ctx, expression),
        ConditionalType::Else => VariableValue::Bool(true),
    } {
        if should_run && ctx.continue_to_next_if {
            interpret_block(ctx.clone(), body);
            ctx.continue_to_next_if = false
        }
    }
}

fn declare_function(
    ctx: &mut ExecutionContext,
    _return_type: VarType,
    name: String,
    _params: Vec<Parameter>,
    body: Vec<Statement>,
) {
    ctx.callable_functions.insert(name.clone());
    ctx.declared_functions.insert(
        name,
        FunctionData {
            ctx: ctx.clone(),
            ast: body,
        },
    );
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
            Literal::Int(new_int) => VariableValue::Int(new_int),
            Literal::Bool(new_bool) => VariableValue::Bool(new_bool),
            _ => todo!(),
        },
        Expression::BinaryOperation { left, op, right } => {
            if let VariableValue::Int(left) = eval(ctx, *left)
                && let VariableValue::Int(right) = eval(ctx, *right)
            {
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
            } else {
                todo!()
            }
        }
        _ => todo!(),
    }
}
