use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use crate::ast::ast_nodes::*;
use context::*;

mod assignment_ops;
mod binary_ops;
mod context;
mod declarations;
mod functions;
mod loops;
mod macros;
mod unary_ops;

pub fn interpret(ast: Vec<Statement>) {
    interpret_block(ExecutionContext::default(), &ast);
}

fn interpret_block(mut ctx: ExecutionContext, ast: &[Statement]) -> Option<VariableValue> {
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
            Statement::Expression(expr) => {
                eval(&mut ctx, expr);
            }
            Statement::Conditional {
                conditional_type,
                body,
            } => {
                if let Some(early_return) = handle_conditionals(&mut ctx, conditional_type, body) {
                    return Some(early_return);
                }
            }
            Statement::Loop {
                loop_over,
                index_var,
                body,
            } => {
                if let Some(early_return) = loops::eval_loop(&mut ctx, loop_over, index_var, body) {
                    return Some(early_return);
                }
            }
            Statement::Return(expression) => return Some(eval(&mut ctx, expression)),
        }
    }
    // dbg!(ctx.declared_variables);
    None
}

fn handle_conditionals(
    ctx: &mut ExecutionContext,
    conditional_type: &ConditionalType,
    body: &[Statement],
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
            Literal::String(new_str) => VariableValue::Str(new_str.to_string()),
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
        Expression::UnaryOperation { op, expr } => unary_ops::eval_unary_ops(ctx, op, expr),
        Expression::FunctionCall { name, args } => functions::run_function(ctx, name, args),
        Expression::CompositeLiteral(comp_literal) => match comp_literal {
            CompositeLiteral::Vec(expressions) => {
                VariableValue::Vec(expressions.iter().map(|expr| eval(ctx, expr)).collect())
            }
            _ => todo!(),
        },
        Expression::Conversion { to, expr } => functions::convert(ctx, to, expr),
        _ => todo!(),
    }
}
