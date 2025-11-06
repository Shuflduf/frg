use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::{ast::ast_nodes::*, interpreter::functions::run_function};
use context::*;

mod assignment_ops;
mod binary_ops;
mod context;
mod declarations;
mod functions;

fn interpret_block(mut ctx: ExecutionContext, ast: &Vec<Statement>) -> Option<VariableValue> {
    // println!("RUNNING BLOCK {:#?}", &ctx);
    for statement in ast {
        match statement {
            Statement::VariableDeclaration {
                var_type,
                name,
                value,
            } => {
                let value = eval(&mut ctx, value);
                declarations::declare_variable(&mut ctx, var_type, name, value);
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                params,
                body,
            } => declarations::declare_function(&mut ctx, return_type, name, params, body),
            Statement::StructDeclaration { name, fields } => {
                declarations::declare_struct(&mut ctx, name, fields)
            }
            Statement::Assignment { name, value, op } => {
                assignment_ops::eval_assignment_ops(&mut ctx, name, value, op)
            }
            Statement::Conditional {
                conditional_type,
                body,
            } => {
                if let Some(early_return) = handle_conditionals(&mut ctx, conditional_type, body) {
                    return Some(early_return);
                }
            }
            Statement::Return(expression) => return Some(eval(&mut ctx, expression)),
            _ => todo!(),
        }
    }
    dbg!(ctx.declared_variables);
    None
}

pub fn interpret(ast: Vec<Statement>) {
    interpret_block(ExecutionContext::default(), &ast);
}

fn handle_conditionals(
    ctx: &mut ExecutionContext,
    conditional_type: &ConditionalType,
    body: &Vec<Statement>,
) -> Option<VariableValue> {
    // println!("CONDITIONAL {:#?}", &ctx);
    if let VariableValue::Bool(should_run) = match conditional_type {
        ConditionalType::If(expression) => {
            // dbg!(&expression);
            ctx.continue_to_next_if = true;
            eval(ctx, expression)
        }
        ConditionalType::Elif(expression) => eval(ctx, expression),
        ConditionalType::Else => VariableValue::Bool(true),
    } {
        if should_run && ctx.continue_to_next_if {
            if let Some(early_return) = interpret_block(ctx.clone(), body) {
                return Some(early_return);
            }
            ctx.continue_to_next_if = false;
        }
        None
    } else {
        panic!("use bools dumbass")
    }
}

fn eval(ctx: &mut ExecutionContext, expression: &Expression) -> VariableValue {
    // println!("EVALING {:#?} {:#?}", &expression, &ctx.declared_variables);
    match expression {
        Expression::Literal(literal) => match literal {
            Literal::Int(new_int) => VariableValue::Int(*new_int),
            Literal::Bool(new_bool) => VariableValue::Bool(*new_bool),
            Literal::Float(new_float) => VariableValue::Float(*new_float),
            _ => todo!(),
        },
        Expression::Identifier(identifier) => ctx
            .declared_variables
            .get(identifier)
            .unwrap_or_else(|| {
                panic!(
                    "variable `{identifier}` doesnt exist {:?}",
                    &ctx.declared_variables
                )
            })
            .borrow()
            .value
            .clone(),
        Expression::BinaryOperation { left, op, right } => {
            binary_ops::eval_binary_ops(ctx, left, op, right)
        }
        Expression::FunctionCall { name, args } => run_function(ctx, name, args),
        _ => todo!(),
    }
}
