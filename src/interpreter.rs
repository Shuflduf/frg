use std::{cell::RefCell, collections::HashMap, rc::Rc};

use chumsky::span::{SimpleSpan, Spanned};

use crate::{
    ast::{
        AssignmentOperator, BinaryOperator, Expr, Expression, Literal, Statement, Stmt,
        UnaryOperator, VarType,
    },
    errors::{Diagnostic, OpError},
    interpreter::value::FunctionValue,
};
use value::Value;

mod binary_operation;
mod if_statement;
mod index_access;
mod member_access;
mod struct_literal;
mod unary_operation;
mod value;
mod variable_assignment;
mod variable_declaration;

const STD: Expression = Expression::Identifier("std");

#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug)]
pub(crate) struct Slot<'ast> {
    vtype_name: Spanned<&'ast str>,
    vtype: TypeValue<'ast>,
    value: Value<'ast>,
}

type SlotRef<'ast> = Rc<RefCell<Slot<'ast>>>;

#[derive(Debug)]
pub(crate) struct Variable<'ast> {
    slot: SlotRef<'ast>,
}
impl<'ast> Clone for Slot<'ast> {
    fn clone(&self) -> Self {
        Self {
            vtype_name: self.vtype_name,
            vtype: self.vtype.clone(),
            value: self.value.clone(),
        }
    }
}

impl<'ast> Clone for Variable<'ast> {
    fn clone(&self) -> Self {
        Self {
            slot: Rc::new(RefCell::new(self.slot.borrow().clone())),
        }
    }
}

impl<'ast> Variable<'ast> {
    fn new(vtype_name: Spanned<&'ast str>, vtype: TypeValue<'ast>, value: Value<'ast>) -> Self {
        Self {
            slot: Rc::new(RefCell::new(Slot {
                vtype_name,
                vtype,
                value,
            })),
        }
    }

    fn value(&self) -> Value<'ast> {
        self.slot.borrow().value.clone()
    }
}

#[derive(Debug, Clone, Default)]
struct Scope<'ast> {
    variables: HashMap<String, Variable<'ast>>,
}

#[derive(Debug, Clone)]
struct State<'ast> {
    scopes: Vec<Scope<'ast>>,
}

impl<'ast> State<'ast> {
    fn new() -> Self {
        Self {
            scopes: vec![Scope::default()],
        }
    }

    fn lookup(&self, name: &str) -> Option<&Variable<'ast>> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.variables.get(name))
    }

    fn declare(&mut self, name: &str, variable: Variable<'ast>) {
        self.scopes
            .last_mut()
            .expect("state should have at least one scope")
            .variables
            .insert(name.to_string(), variable);
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope::default());
    }

    fn pop_scope(&mut self) -> Scope<'ast> {
        self.scopes
            .pop()
            .expect("state should have at least one scope")
    }
}

struct LValue<'ast> {
    slot: SlotRef<'ast>,
    vtype: TypeValue<'ast>,
    vtype_name: Spanned<&'ast str>,
}

pub fn run<'ast>(program: &[Stmt<'ast>]) -> Result<Option<Value<'ast>>, Diagnostic> {
    let mut state = State::new();

    exec_statements(&mut state, program)
}

fn deref_value<'ast>(
    state: &mut State<'ast>,
    expr: &Expr<'ast>,
    span: SimpleSpan,
) -> Result<Value<'ast>, Diagnostic> {
    let mut value = eval(state, expr)?;
    for _ in 0..64 {
        match value {
            Value::Reference(slot) => value = slot.borrow().value.clone(),
            other => return Ok(other),
        }
    }
    Err(Diagnostic::basic(
        "reference chain is too deep (max 64)",
        span,
    ))
}

fn exec_block<'ast>(
    state: &mut State<'ast>,
    stmts: &[Stmt<'ast>],
) -> Result<Option<Value<'ast>>, Diagnostic> {
    state.push_scope();
    let res = exec_statements(state, stmts);
    state.pop_scope();
    res
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
            Statement::If {
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
            Statement::Return(expr) => {
                return expr.as_ref().map(|expr| eval(state, &expr)).transpose();
            }
            Statement::StandaloneExpression(expr) => {
                eval(state, expr)?;
            }
            Statement::Block(body) => {
                if let Some(value) = exec_block(state, body)? {
                    return Ok(Some(value));
                }
            }
        }
    }

    Ok(None)
}

fn eval<'ast>(state: &mut State<'ast>, expr: &Expr<'ast>) -> Result<Value<'ast>, Diagnostic> {
    match &expr.inner {
        Expression::Identifier(name) => state
            .lookup(name)
            .map(Variable::value)
            .ok_or_else(|| Diagnostic::basic(format!("undefined variable `{name}`"), expr.span)),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => Ok(Value::Int(*n)),
            Literal::Float(n) => Ok(Value::Float(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            Literal::Str(s) => Ok(Value::String(s.to_string())),
            Literal::Undefined => Ok(Value::Undefined),
        },
        Expression::TypeLiteral(vtype) => Ok(Value::Type(eval_type(state, vtype)?)),
        Expression::ModuleLiteral(body) => {
            state.push_scope();
            let result = exec_statements(state, body);
            let scope = state.pop_scope();
            result?;
            Ok(Value::Module(scope.variables))
        }
        Expression::BinaryOperation { left, op, right } => {
            binary_operation::eval(state, expr.span, left, op, right)
        }
        Expression::UnaryOperation { op, right } => {
            unary_operation::eval(state, expr.span, op, right)
        }
        Expression::MemberAccess { parent, member } => {
            member_access::eval(state, expr.span, parent, *member)
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
            index_access::eval(state, expr.span, parent, index)
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
        // let reciever = method_reciever(state, parent)?;
        let parent_value = deref_value(state, parent, parent.span)?;
        if let Value::Module(members) = parent_value {
            let callee = members
                .get(member.inner)
                .map(Variable::value)
                .ok_or_else(|| {
                    Diagnostic::basic(format!("no member `{}`", member.inner), member.span)
                })?;
            return call_value(state, span, callee, params, None);
        }
        let reciever = method_reciever(state, parent, parent_value)?;
        let callee = Spanned {
            inner: Expression::Identifier(member.inner),
            span: member.span,
        };
        return eval_call(state, span, &callee, params, Some(reciever));
    }
    let callee = eval(state, function)?;
    call_value(state, span, callee, params, implicit)
}

fn call_value<'ast>(
    state: &mut State<'ast>,
    span: SimpleSpan,
    callee: Value<'ast>,
    params: &[Expr<'ast>],
    implicit: Option<Value<'ast>>,
) -> Result<Value<'ast>, Diagnostic> {
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
                local.declare(
                    name,
                    Variable::new(Spanned { inner: *name, span }, type_of_value(&value), value),
                );
            }
            Ok(exec_statements(&mut local, &function.body)?.unwrap_or(Value::Undefined))
        }
        Value::Type(TypeValue::List(elem)) => Ok(Value::Type(TypeValue::List(elem))),
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
    state: &State<'ast>,
    parent: &Expr<'ast>,
    value: Value<'ast>,
) -> Result<Value<'ast>, Diagnostic> {
    if let Expression::Identifier(name) = parent.inner {
        let variable = state.lookup(name).ok_or_else(|| {
            Diagnostic::basic(format!("undefined variable `{name}`"), parent.span)
        })?;
        return Ok(Value::Reference(Rc::clone(&variable.slot)));
    }
    match value {
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
        VarType::Custom(expr) => match eval(state, expr)? {
            Value::Type(vtype) => Ok(vtype),
            other => Err(Diagnostic::basic(
                format!("expected a type, found `{}`", other.type_name()),
                expr.span,
            )),
        },
        VarType::Module => Ok(TypeValue::Primitive(VarType::Module)),
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
                .unwrap_or_else(|| TypeValue::Primitive(VarType::Void));
            TypeValue::List(Box::new(first))
        }
        Value::Struct(fields) => TypeValue::Struct(
            fields
                .values()
                .map(|var| {
                    let slot = var.slot.borrow();
                    (slot.vtype.clone(), slot.vtype_name.inner)
                })
                .collect(),
        ),
        Value::Module(_) => TypeValue::Primitive(VarType::Module),
        Value::Reference(slot) => TypeValue::Reference(Box::new(slot.borrow().vtype.clone())),
        // TODO
        Value::Function(_) => TypeValue::Function {
            returns: Box::new(TypeValue::Primitive(VarType::Void)),
            params: Vec::new(),
        },
        // Value::M
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
        (TypeValue::Primitive(VarType::Module), Value::Module(_)) => true,
        (TypeValue::Primitive(_), _) => false,
        (TypeValue::List(elem), Value::List(values)) => {
            values.iter().all(|value| value_matches_type(elem, value))
        }
        (TypeValue::Struct(fields), Value::Struct(values)) => {
            fields.len() == values.len()
                && fields.iter().all(|(field_type, field_name)| {
                    values.get(*field_name).is_some_and(|var| {
                        let slot = var.slot.borrow();
                        value_matches_type(field_type, &slot.value)
                    })
                })
        }
        // TODO
        (TypeValue::Function { .. }, Value::Function(_)) => true,
        (TypeValue::Reference(inner), Value::Reference(slot)) => {
            let slot = slot.borrow();
            slot.vtype == **inner || value_matches_type(inner, &slot.value)
        }
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

fn resolve_lvalue<'ast>(
    state: &mut State<'ast>,
    expr: &Expr<'ast>,
) -> Result<LValue<'ast>, Diagnostic> {
    match &expr.inner {
        Expression::Identifier(name) => {
            let variable = state.lookup(*name).ok_or_else(|| {
                Diagnostic::basic(format!("undefined variable `{name}`"), expr.span)
            })?;
            let slot = Rc::clone(&variable.slot);
            let (vtype, vtype_name) = {
                let slot = slot.borrow();
                (slot.vtype.clone(), slot.vtype_name)
            };
            Ok(LValue {
                slot,
                vtype,
                vtype_name,
            })
        }
        Expression::UnaryOperation {
            op: UnaryOperator::Dereference,
            right,
        } => match eval(state, right)? {
            Value::Reference(slot) => {
                let (vtype, vtype_name) = {
                    let slot = slot.borrow();
                    let vtype = match &slot.vtype {
                        TypeValue::Reference(inner) => (**inner).clone(),
                        other => other.clone(),
                    };
                    (vtype, slot.vtype_name)
                };
                Ok(LValue {
                    slot,
                    vtype,
                    vtype_name,
                })
            }
            // Expression::Identifier(name) => name.to_string(),
            other => {
                return Err(Diagnostic::basic(
                    format!("cannot assign through `{}`", other.type_name()),
                    expr.span,
                ));
            }
        },
        _ => {
            return Err(Diagnostic::basic(
                "cannot assign to this expression",
                expr.span,
            ));
        }
    }
}
