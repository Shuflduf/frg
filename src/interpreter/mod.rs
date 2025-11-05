use std::collections::{HashMap, HashSet};

use crate::ast::ast_nodes::*;

#[derive(Debug, Clone)]
struct VariableData {
    var_type: VarType,
    value: VariableValue,
}

#[derive(Debug, Clone)]
enum VariableValue {
    Void,
    Int(i32),
    Bool(bool),
}

#[derive(Debug, Clone)]
struct FunctionData {
    ctx: ExecutionContext,
    ast: Vec<Statement>,
    params: Vec<Parameter>,
    return_type: VarType,
}

#[derive(Debug, Default, Clone)]
struct ExecutionContext {
    declared_variables: HashMap<String, Box<VariableData>>,
    declared_functions: HashMap<String, Box<FunctionData>>,
    // maybe use a tuple struct for this
    declared_structs: HashMap<String, Box<HashMap<String, VarType>>>,
    continue_to_next_if: bool,
}

fn interpret_block(mut ctx: ExecutionContext, ast: Vec<Statement>) -> VariableValue {
    // println!("RUNNING BLOCK {:#?}", &ctx);
    for statement in ast {
        match statement {
            Statement::VariableDeclaration {
                var_type,
                name,
                value,
            } => {
                let value = eval(&mut ctx, value);
                declare_variable(&mut ctx, var_type, name, value);
            }
            Statement::FunctionDeclaration {
                return_type,
                name,
                params,
                body,
            } => declare_function(&mut ctx, return_type, name, params, body),
            Statement::StructDeclaration { name, fields } => declare_struct(&mut ctx, name, fields),
            Statement::Conditional {
                conditional_type,
                body,
            } => handle_conditionals(&mut ctx, conditional_type, body),
            Statement::Return(expression) => return eval(&mut ctx, expression),
            _ => todo!(),
        }
    }
    dbg!(ctx);
    VariableValue::Void
}

pub fn interpret(ast: Vec<Statement>) {
    interpret_block(ExecutionContext::default(), ast);
}

fn handle_conditionals(
    ctx: &mut ExecutionContext,
    conditional_type: ConditionalType,
    body: Vec<Statement>,
) {
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
            interpret_block(ctx.clone(), body);
            ctx.continue_to_next_if = false
        }
    } else {
        panic!("use bools dumbass")
    }
}

fn declare_function(
    ctx: &mut ExecutionContext,
    return_type: VarType,
    name: String,
    params: Vec<Parameter>,
    body: Vec<Statement>,
) {
    ctx.declared_functions.insert(
        name,
        Box::new(FunctionData {
            ctx: ctx.clone(),
            ast: body,
            params,
            return_type,
        }),
    );
}

fn declare_variable(
    ctx: &mut ExecutionContext,
    var_type: VarType,
    name: String,
    value: VariableValue,
) {
    ctx.declared_variables
        .insert(name, Box::new(VariableData { value, var_type }));
}

fn declare_struct(ctx: &mut ExecutionContext, name: String, fields: Vec<Parameter>) {
    let mut struct_fields = HashMap::new();
    fields.iter().for_each(|param| {
        struct_fields.insert(param.name.clone(), param.param_type.clone());
    });
    ctx.declared_structs.insert(name, Box::new(struct_fields));
}

fn eval(ctx: &mut ExecutionContext, expression: Expression) -> VariableValue {
    // println!("EVALING {:#?} {:#?}", &expression, &ctx.declared_variables);
    match expression {
        Expression::Literal(literal) => match literal {
            Literal::Int(new_int) => VariableValue::Int(new_int),
            Literal::Bool(new_bool) => VariableValue::Bool(new_bool),
            _ => todo!(),
        },
        Expression::Identifier(identifier) => ctx
            .declared_variables
            .get(&identifier)
            .expect(&format!(
                "variable `{identifier}` doesnt exist {:?}",
                &ctx.declared_variables
            ))
            .value
            .clone(),
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
        Expression::FunctionCall { name, args } => {
            let params: Vec<VariableValue> =
                args.iter().map(|exp| eval(ctx, exp.clone())).collect();
            let target_func = ctx
                .declared_functions
                .get(&name)
                .expect(&format!("function `{name}` doesnt exist"));
            let mut func_ctx = target_func.ctx.clone();
            func_ctx
                .declared_functions
                .insert(name.clone(), target_func.clone());
            target_func
                .params
                .iter()
                .enumerate()
                .for_each(|(i, param)| {
                    println!("adding var `{}` to `{}`", param.name.clone(), name.clone());
                    declare_variable(
                        &mut func_ctx,
                        param.param_type.clone(),
                        param.name.clone(),
                        params[i].clone(),
                    );
                });
            interpret_block(func_ctx, target_func.ast.clone())
        }
        _ => todo!(),
    }
}
