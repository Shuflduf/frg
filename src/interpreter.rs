use std::collections::HashMap;

use chumsky::span::{SimpleSpan, Spanned};

use crate::{
    ast::{
        AssignmentOperator, BinaryOperator, Expr, Expression, Literal, Statement, Stmt,
        UnaryOperator, VarType,
    },
    errors::{Diagnostic, OpError},
};
use value::Value;

mod binary_operation;
mod if_statement;
mod struct_literal;
mod unary_operation;
mod value;
mod variable_assignment;
mod variable_declaration;

const STD: Expression = Expression::Identifier("std");

#[derive(Debug, Clone)]
enum CustomType<'ast> {
    StructType(Vec<(VarType<'ast>, &'ast str)>),
}

#[derive(Debug, Clone)]
struct Variable<'ast> {
    vtype_name: Spanned<&'ast str>,
    vtype: VarType<'ast>,
    value: Value<'ast>,
}
#[derive(Debug)]
struct State<'ast> {
    variables: HashMap<String, Variable<'ast>>,
}

pub fn run(program: &[Stmt]) -> Result<i64, Diagnostic> {
    let mut state = State {
        variables: HashMap::new(),
    };

    let result = run_statements(&mut state, program);

    println!("{state:#?}");

    result
}

fn run_statements<'ast>(
    state: &mut State<'ast>,
    stmts: &'ast [Stmt<'_>],
) -> Result<i64, Diagnostic> {
    for stmt in stmts {
        match &stmt.inner {
            Statement::VariableDeclaration {
                vtype_name,
                vtype,
                name,
                value,
            } => variable_declaration::eval(state, vtype_name, vtype, name, value)?,
            Statement::IfStatement {
                condition,
                body,
                else_ifs,
                else_body,
            } => if_statement::eval(state, condition, body, else_ifs, else_body)?,
            Statement::VariableAssignment { variable, op, expr } => {
                variable_assignment::eval(state, variable, op, expr)?
            }
            #[allow(unreachable_patterns)]
            _ => todo!(),
        }
    }

    Ok(0)
}

fn eval<'ast>(state: &State<'ast>, expr: &Expr<'ast>) -> Result<Value<'ast>, Diagnostic> {
    match &expr.inner {
        Expression::Identifier(name) => state
            .variables
            .get(&name.to_string())
            .ok_or_else(|| Diagnostic::basic(format!("undefined variable `{name}`"), expr.span))
            .map(|e| e.value.clone()),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => Ok(Value::Int(*n)),
            Literal::Float(n) => Ok(Value::Float(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Str(s) => Ok(Value::String(s.to_string())),
            #[allow(unreachable_patterns)]
            _ => todo!(),
        },
        Expression::BinaryOperation { left, op, right } => {
            binary_operation::eval(state, expr.span, left, op, right)
        }
        Expression::UnaryOperation { op, right } => {
            unary_operation::eval(state, expr.span, op, right)
        }
        Expression::MemberAccess { parent, member } => {
            match &parent.inner {
                &STD => {
                    if member.inner == "debug" {
                        return Ok(Value::Int(1));
                    }
                }
                Expression::StructLiteral {
                    parent: _struct_name,
                    fields,
                } => {
                    return Ok(eval(
                        state,
                        &fields
                            .iter()
                            .find(|(name, _)| *name == member.inner)
                            .unwrap()
                            .1,
                    )?);
                }
                _other => {} // Expression::Identifier(id) => get_variable(&mut state, id),
                             // other => {
                             //     match get_variable(&mut state, other) {}
                             //     todo!("{other:?}")
                             // }
            }
            match eval(state, parent)? {
                Value::Struct(vars) => return Ok(vars.get(member.inner).unwrap().value.clone()),

                other => todo!("{other:?}"),
            };
            // println!("{parent:?}");
            Err(Diagnostic::new("shit"))
        }
        Expression::StructTypeDeclaration { fields } => Ok(Value::Type(CustomType::StructType(
            fields
                .iter()
                .map(|field: &(VarType, Spanned<&str>, &str)| (field.0.clone(), field.2))
                .collect::<Vec<_>>(),
        ))),
        Expression::StructLiteral { parent, fields } => struct_literal::eval(state, parent, fields),
        _ => todo!(),
    }
}

fn get_variable<'a, 'state>(
    state: &'a mut State<'state>,
    expr: &Expr,
) -> Result<&'a mut Variable<'state>, Diagnostic> {
    match **expr {
        Expression::Identifier(identifier) => state
            .variables
            .get_mut(identifier)
            .ok_or(Diagnostic::new("var not found")),
        _ => Err(Diagnostic::new("shit")),
    }
}

fn get_type<'a, 'state>(
    state: &'a State<'state>,
    expr: &Expr,
) -> Result<CustomType<'state>, Diagnostic> {
    match **expr {
        Expression::Identifier(identifier) => match state
            .variables
            .get(identifier)
            .ok_or(Diagnostic::new("var not found"))
            .map(|var| &var.value)
        {
            Ok(Value::Type(vtype)) => Ok(vtype.clone()),
            _ => Err(Diagnostic::new("this ts not type")),
        },
        _ => Err(Diagnostic::new("var doesnt exist")),
    }
}

fn var_type_check(vtype: &VarType, value: &Value) -> bool {
    matches!(
        (vtype, value),
        (VarType::Int, Value::Int(_))
            | (VarType::Float, Value::Float(_))
            | (VarType::Bool, Value::Bool(_))
            | (VarType::Str, Value::String(_))
            | (VarType::Type, Value::Type(_))
            | (VarType::Custom(_), Value::Struct(_))
    )
}
