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
pub enum TypeValue<'ast> {
    Primitive(VarType<'ast>),
    List(Box<TypeValue<'ast>>),
    Struct(Vec<(TypeValue<'ast>, &'ast str)>),
    Function {
        returns: Box<TypeValue<'ast>>,
        params: Vec<TypeValue<'ast>>,
    },
    Reference(Box<TypeValue<'ast>>),
}

#[derive(Debug, Clone)]
struct FunctionValue<'ast> {
    params: Vec<&'ast str>,
    body: Vec<Stmt<'ast>>,
}

#[derive(Debug, Clone)]
struct Variable<'ast> {
    vtype_name: Spanned<&'ast str>,
    vtype: TypeValue<'ast>,
    value: Value<'ast>,
}
#[derive(Debug, Clone)]
struct State<'ast> {
    variables: HashMap<String, Variable<'ast>>,
}

pub fn run<'ast>(program: &[Stmt<'ast>]) -> Result<i64, Diagnostic> {
    let mut state = State {
        variables: HashMap::new(),
    };

    let result = exec_statements(&mut state, program).map(|_| 0);

    println!("{state:#?}");

    result
}

fn exec_statements<'ast>(
    state: &mut State<'ast>,
    stmts: &[Stmt<'ast>],
) -> Result<Option<Value<'ast>>, Diagnostic> {
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
            } => {
                if let Some(value) =
                    if_statement::eval(state, condition, body, else_ifs, else_body)?
                {
                    return Ok(Some(value));
                }
            }
            Statement::VariableAssignment { variable, op, expr } => {
                variable_assignment::eval(state, variable, op, expr)?
            }
            Statement::Return(expr) => return Ok(Some(eval(state, expr)?)),
            Statement::StandaloneExpression(expr) => {
                eval(state, expr)?;
            }
        }
    }

    Ok(None)
}

fn eval<'ast>(state: &mut State<'ast>, expr: &Expr<'ast>) -> Result<Value<'ast>, Diagnostic> {
    match &expr.inner {
        Expression::Identifier(name) => state
            .variables
            .get(*name)
            .map(|var| var.value.clone())
            .ok_or_else(|| Diagnostic::basic(format!("undefined variable `{name}`"), expr.span)),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => Ok(Value::Int(*n)),
            Literal::Float(n) => Ok(Value::Float(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Str(s) => Ok(Value::String(s.to_string())),
            Literal::Undefined => Ok(Value::Undefined),
        },
        Expression::TypeLiteral(vtype) => Ok(Value::Type(eval_type(state, vtype)?)),
        Expression::BinaryOperation { left, op, right } => {
            binary_operation::eval(state, expr.span, left, op, right)
        }
        Expression::UnaryOperation { op, right } => {
            unary_operation::eval(state, expr.span, op, right)
        }
        Expression::MemberAccess { parent, member } => {
            if matches!(&parent.inner, &STD) && member.inner == "debug" {
                return Ok(Value::Int(1));
            }
            match eval(state, parent)? {
                Value::Struct(vars) => vars
                    .get(member.inner)
                    .map(|var| var.value.clone())
                    .ok_or_else(|| {
                        Diagnostic::basic(format!("no field `{}`", member.inner), member.span)
                    }),
                Value::Reference(target) => {
                    match state.variables.get(&target).map(|var| &var.value) {
                        Some(Value::Struct(vars)) => vars
                            .get(member.inner)
                            .map(|var| var.value.clone())
                            .ok_or_else(|| {
                                Diagnostic::basic(
                                    format!("no field `{}`", member.inner),
                                    member.span,
                                )
                            }),
                        _ => Err(Diagnostic::basic(
                            format!("no field `{}`", member.inner),
                            member.span,
                        )),
                    }
                }
                other => Err(Diagnostic::basic(
                    format!(
                        "cannot access the field `{}` on `{}`",
                        member.inner,
                        other.type_name()
                    ),
                    member.span,
                )),
            }
        }
        Expression::StructTypeDeclaration { fields } => {
            let mut resolved = Vec::with_capacity(fields.len());
            for (vtype, _, name) in fields {
                resolved.push((eval_type(state, vtype)?, *name));
            }

            if resolved.is_empty() {
                return Err(Diagnostic::basic(
                    "struct needs at least one field",
                    expr.span,
                ));
            }
            Ok(Value::Type(TypeValue::Struct(resolved)))
        }
        Expression::StructLiteral { parent, fields } => struct_literal::eval(state, parent, fields),
        Expression::IndexAccess { parent, index } => {
            let parent_value = eval(state, parent)?;
            let index_value = eval(state, index)?;
            let index_span = index.span;
            let index = match index_value {
                Value::Int(n) if n >= 0 => n as usize,
                other => {
                    return Err(Diagnostic::basic(
                        format!(
                            "index must be a non-negative int, found `{}`",
                            other.type_name()
                        ),
                        index_span,
                    ));
                }
            };
            match parent_value {
                Value::List(values) => values
                    .into_iter()
                    .nth(index)
                    .ok_or_else(|| Diagnostic::basic("index out of bounds", index_span)),
                other => Err(Diagnostic::basic(
                    format!("cannot index into `{}`", other.type_name()),
                    parent.span,
                )),
            }
        }
        Expression::FunctionLiteral { params, body } => Ok(Value::Function(FunctionValue {
            params: params.clone(),
            body: body.clone(),
        })),
        Expression::VecLiteral(values) => {
            let values = values
                .iter()
                .map(|value| eval(state, value))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::List(values))
        }
        Expression::FunctionCall { function, params } => {
            eval_call(state, expr.span, function, params, None)
        }
        _ => todo!(),
    }
}

fn eval_call<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    function: &Expr<'ast>,
    params: &[Expr<'ast>],
    implicit: Option<Value<'ast>>,
) -> Result<Value<'ast>, Diagnostic> {
    if let Expression::MemberAccess { parent, member } = &function.inner {
        let reciever = method_reciever(state, parent)?;
        let callee = Spanned {
            inner: Expression::Identifier(member.inner),
            span: member.span,
        };
        return eval_call(state, span, &callee, params, Some(reciever));
    }
    let callee = eval(state, function)?;
    match callee {
        Value::Function(function) => {
            let positional = params.len() + if implicit.is_some() { 1 } else { 0 };
            if function.params.len() != positional {
                return Err(Diagnostic::basic(
                    format!(
                        "expected {} argument(s), found {}",
                        function.params.len(),
                        positional
                    ),
                    span,
                ));
            }
            let mut arguments = Vec::with_capacity(positional);
            if let Some(value) = implicit {
                arguments.push(value);
            }
            for param in params {
                arguments.push(eval(state, param)?);
            }
            let mut local = state.clone();
            for (name, value) in function.params.iter().zip(arguments) {
                local.variables.insert(
                    name.to_string(),
                    Variable {
                        vtype_name: Spanned { inner: *name, span },
                        vtype: type_of_value(&value),
                        value,
                    },
                );
            }
            Ok(exec_statements(&mut local, &function.body)?.unwrap_or(Value::Undefined))
        }
        Value::Type(TypeValue::Primitive(VarType::List)) => {
            let type_args = type_arguments(state, span, implicit, params)?;
            if type_args.len() != 1 {
                return Err(Diagnostic::basic("vec expects one element of `type`", span));
            }
            Ok(Value::Type(TypeValue::List(Box::new(
                type_args.into_iter().next().unwrap(),
            ))))
        }
        Value::Type(vtype) => {
            let type_args = type_arguments(state, span, implicit, params)?;
            Ok(Value::Type(TypeValue::Function {
                returns: Box::new(vtype),
                params: type_args,
            }))
        }
        other => Err(Diagnostic::basic(
            format!("cannot call `{}`", other.type_name()),
            span,
        )),
    }
}

fn method_reciever<'ast>(
    state: &mut State<'ast>,
    parent: &Expr<'ast>,
) -> Result<Value<'ast>, Diagnostic> {
    if let Expression::Identifier(name) = parent.inner {
        return Ok(Value::Reference(name.to_string()));
    }
    match eval(state, parent)? {
        Value::Reference(value) => Ok(Value::Reference(value)),
        other => Err(Diagnostic::basic(
            format!(
                "method reciever must be referenceable, found `{}`",
                other.type_name()
            ),
            parent.span,
        )),
    }
}

fn eval_type<'ast>(
    state: &mut State<'ast>,
    vtype: &VarType<'ast>,
) -> Result<TypeValue<'ast>, Diagnostic> {
    match vtype {
        VarType::Void => Ok(TypeValue::Primitive(VarType::Void)),
        VarType::Int => Ok(TypeValue::Primitive(VarType::Int)),
        VarType::Float => Ok(TypeValue::Primitive(VarType::Float)),
        VarType::Str => Ok(TypeValue::Primitive(VarType::Str)),
        VarType::Bool => Ok(TypeValue::Primitive(VarType::Bool)),
        VarType::Type => Ok(TypeValue::Primitive(VarType::Type)),
        VarType::List => Ok(TypeValue::Primitive(VarType::List)),
        VarType::Custom(expr) => match eval(state, expr)? {
            Value::Type(vtype) => Ok(vtype),
            other => Err(Diagnostic::basic(
                format!("expected a type, found `{}`", other.type_name()),
                expr.span,
            )),
        },
    }
}

fn type_of_value<'ast>(value: &Value<'ast>) -> TypeValue<'ast> {
    match value {
        Value::Undefined => TypeValue::Primitive(VarType::Void),
        Value::Int(_) => TypeValue::Primitive(VarType::Int),
        Value::Float(_) => TypeValue::Primitive(VarType::Float),
        Value::Bool(_) => TypeValue::Primitive(VarType::Bool),
        Value::String(_) => TypeValue::Primitive(VarType::Str),
        Value::Type(_) => TypeValue::Primitive(VarType::Type),
        Value::List(values) => {
            let first = values
                .first()
                .map(type_of_value)
                .unwrap_or_else(|| TypeValue::Primitive(VarType::List));
            TypeValue::List(Box::new(first))
        }
        Value::Struct(fields) => TypeValue::Struct(
            fields
                .values()
                .map(|var| (var.vtype.clone(), var.vtype_name.inner))
                .collect(),
        ),
        // TODO
        Value::Function(_) => TypeValue::Function {
            returns: Box::new(TypeValue::Primitive(VarType::Void)),
            params: Vec::new(),
        },
        Value::Reference(_) => TypeValue::Reference(Box::new(TypeValue::Primitive(VarType::List))),
    }
}

fn value_matches_type<'ast>(vtype: &TypeValue<'ast>, value: &Value<'ast>) -> bool {
    match (vtype, value) {
        (TypeValue::Primitive(VarType::Void), Value::Undefined) => true,
        (TypeValue::Primitive(VarType::Int), Value::Int(_)) => true,
        (TypeValue::Primitive(VarType::Float), Value::Float(_)) => true,
        (TypeValue::Primitive(VarType::Bool), Value::Bool(_)) => true,
        (TypeValue::Primitive(VarType::Str), Value::String(_)) => true,
        (TypeValue::Primitive(VarType::Type), Value::Type(_)) => true,
        (TypeValue::Primitive(VarType::List), Value::List(_)) => true,
        (TypeValue::Primitive(_), _) => false,
        (TypeValue::List(elem), Value::List(values)) => {
            values.iter().all(|value| value_matches_type(elem, value))
        }
        (TypeValue::Struct(fields), Value::Struct(values)) => {
            fields.len() == values.len()
                && fields.iter().all(|(field_type, field_name)| {
                    values
                        .get(*field_name)
                        .is_some_and(|var| value_matches_type(field_type, &var.value))
                })
        }
        // TODO
        (TypeValue::Function { .. }, Value::Function(_)) => true,
        (TypeValue::Reference(_), Value::Reference(_)) => true,
        _ => false,
    }
}

fn type_arguments<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    implicit: Option<Value<'ast>>,
    params: &[Expr<'ast>],
) -> Result<Vec<TypeValue<'ast>>, Diagnostic> {
    let mut type_args = Vec::with_capacity(params.len() + if implicit.is_some() { 1 } else { 0 });
    if let Some(value) = implicit {
        match value {
            Value::Type(vtype) => type_args.push(vtype),
            other => {
                return Err(Diagnostic::basic(
                    format!("expected a type argument, found `{}`", other.type_name()),
                    span,
                ));
            }
        }
    }
    for param in params {
        type_args.push(eval_type_from_expr(state, param)?);
    }
    Ok(type_args)
}

fn eval_type_from_expr<'ast>(
    state: &mut State<'ast>,
    expr: &Expr<'ast>,
) -> Result<TypeValue<'ast>, Diagnostic> {
    match eval(state, expr)? {
        Value::Type(vtype) => Ok(vtype),
        other => Err(Diagnostic::basic(
            format!("expected a type, found `{}`", other.type_name()),
            expr.span,
        )),
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
