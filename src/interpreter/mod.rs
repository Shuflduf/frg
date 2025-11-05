use std::collections::{HashMap, HashSet};

use crate::ast::ast_nodes::*;

#[derive(Debug, Clone)]
struct VariableData {
    #[allow(dead_code)]
    var_type: VarType,
    value: VariableValue,
}

#[derive(Debug, Clone)]
enum VariableValue {
    Void,
    Int(i32),
    #[allow(dead_code)]
    Str(String),
    #[allow(dead_code)]
    Float(f32),
    Bool(bool),
    #[allow(dead_code)]
    Vec(Vec<VariableValue>),
    #[allow(dead_code)]
    Map(HashMap<VariableValue, VariableValue>),
    #[allow(dead_code)]
    Set(HashSet<VariableValue>),
    #[allow(dead_code)]
    Reference(Box<VariableValue>),
    #[allow(dead_code)]
    Struct(HashMap<String, VariableValue>),
}

#[derive(Debug, Clone)]
struct FunctionData {
    ctx: ExecutionContext,
    ast: Vec<Statement>,
    params: Vec<Parameter>,
    #[allow(dead_code)]
    return_type: VarType,
}

#[derive(Debug, Default, Clone)]
struct ExecutionContext {
    declared_variables: HashMap<String, Box<VariableData>>,
    declared_functions: HashMap<String, Box<FunctionData>>,
    // maybe use a tuple struct for this
    declared_structs: HashMap<String, HashMap<String, VarType>>,
    continue_to_next_if: bool,
}

fn interpret_block(mut ctx: ExecutionContext, ast: Vec<Statement>) -> Option<VariableValue> {
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
    interpret_block(ExecutionContext::default(), ast);
}

fn handle_conditionals(
    ctx: &mut ExecutionContext,
    conditional_type: ConditionalType,
    body: Vec<Statement>,
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
    ctx.declared_structs.insert(name, struct_fields);
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
            .unwrap_or_else(|| {
                panic!(
                    "variable `{identifier}` doesnt exist {:?}",
                    &ctx.declared_variables
                )
            })
            .value
            .clone(),
        Expression::BinaryOperation { left, op, right } => {
            let left = eval(ctx, *left);
            let right = eval(ctx, *right);
            match left {
                VariableValue::Int(left) => {
                    let VariableValue::Int(right) = right else {
                        panic!("cant fucking")
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
                _ => todo!(),
            }
        }
        Expression::FunctionCall { name, args } => {
            let params: Vec<VariableValue> =
                args.iter().map(|exp| eval(ctx, exp.clone())).collect();
            let target_func = ctx
                .declared_functions
                .get(&name)
                .unwrap_or_else(|| panic!("function `{name}` doesnt exist"));
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
            if let Some(returned_value) = interpret_block(func_ctx, target_func.ast.clone()) {
                return returned_value;
            }
            VariableValue::Void
        }
        _ => todo!(),
    }
}
