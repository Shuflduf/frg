use std::{
    collections::HashMap,
    ops::{Add, Neg},
};

use crate::{
    ast::{BinaryOperator, Expr, Expression, Literal, Statement, Stmt, UnaryOperator, VarType},
    errors::{OpError, RuntimeError, RuntimeLabel},
};

#[derive(Debug, Clone)]
enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Value {
    fn type_name(&self) -> &'static str {
        match self {
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
        }
    }
    fn add(self, rhs: Self) -> Result<Self, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + b as f64)),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "+",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
    fn neg(self) -> Result<Self, OpError> {
        match self {
            Self::Int(n) => Ok(Self::Int(-n)),
            Self::Float(n) => Ok(Self::Float(-n)),
            other => Err(OpError::InvalidUnaryOp {
                op: "-",
                expr: other.type_name(),
            }),
        }
    }

    fn equals(&self, rhs: &Self) -> Result<bool, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(a == b),
            (Self::Float(a), Self::Float(b)) => Ok(a == b),
            (Self::Int(a), Self::Float(b)) => Ok(*a as f64 == *b),
            (Self::Float(a), Self::Int(b)) => Ok(*a == *b as f64),
            (Self::Bool(a), Self::Bool(b)) => Ok(a == b),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "==",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, rhs: &Self) -> bool {
        match self {
            Self::Int(lnum) => match rhs {
                Self::Int(rnum) => lnum == rnum,
                Self::Float(rnum) => *lnum as f64 == *rnum,
                Self::Bool(_) => panic!("not allowed"),
            },
            Self::Float(lnum) => match rhs {
                Self::Float(rnum) => lnum == rnum,
                Self::Int(rnum) => *lnum == *rnum as f64,
                Self::Bool(_) => panic!("not allowed"),
            },
            Self::Bool(lbool) => match rhs {
                Self::Bool(rbool) => *lbool == *rbool,
                _ => panic!("not allowed"),
            },
        }
    }
}

#[derive(Debug)]
struct State<'src> {
    variables: HashMap<&'src str, Value>,
}

pub fn run(program: &[Stmt]) -> Result<i64, RuntimeError> {
    let mut state = State {
        variables: HashMap::new(),
    };
    for stmt in program {
        match &stmt.inner {
            Statement::VariableDeclaration { vtype, name, value } => {
                let val = eval(&state, value)?;
                if !var_type_check(vtype, &val) {
                    let expected = var_type_name(vtype);
                    let found = val.type_name();
                    return Err(RuntimeError::with_labels(
                        format!(
                            "variable `{}` expects `{expected}`, got `{found}`",
                            name.inner
                        ),
                        vec![
                            RuntimeLabel {
                                span: name.span,
                                message: format!("declared as `{expected}`"),
                            },
                            RuntimeLabel {
                                span: value.span,
                                message: format!("this is `{found}`"),
                            },
                        ],
                    ));
                }
                state.variables.insert(name.inner, val);
            }
            other => {
                return Err(RuntimeError::new(
                    format!("unsupported statement: {other:?}"),
                    stmt.span,
                ));
            }
        }
    }

    println!("{state:?}");

    Ok(0)
}

fn eval(state: &State, expr: &Expr) -> Result<Value, RuntimeError> {
    match &expr.inner {
        Expression::Identifier(name) => state
            .variables
            .get(name)
            .cloned()
            .ok_or_else(|| RuntimeError::new(format!("undefined variable `{name}`"), expr.span)),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => Ok(Value::Int(*n)),
            Literal::Float(n) => Ok(Value::Float(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            other => Err(RuntimeError::new(
                format!("unsupported literal: {other:?}"),
                expr.span,
            )),
        },
        Expression::BinaryOperation { left, op, right } => {
            let left = eval(state, left)?;
            let right = eval(state, right)?;
            match op {
                BinaryOperator::Add => left
                    .add(right)
                    .map_err(|e| RuntimeError::new(e.to_string(), expr.span)),
                BinaryOperator::Equals => {
                    Ok(Value::Bool(left.equals(&right).map_err(|e| {
                        RuntimeError::new(e.to_string(), expr.span)
                    })?))
                }
                BinaryOperator::NotEquals => {
                    let equal = left
                        .equals(&right)
                        .map_err(|e| RuntimeError::new(e.to_string(), expr.span))?;
                    Ok(Value::Bool(!equal))
                }
                other => Err(RuntimeError::new(
                    format!("unsupported binary operator: {other:?}"),
                    expr.span,
                )),
            }
        }
        Expression::UnaryOperation { op, expr: inner } => {
            let value = eval(state, inner)?;
            match op {
                UnaryOperator::Negative => value
                    .neg()
                    .map_err(|e| RuntimeError::new(e.to_string(), expr.span)),
                other => Err(RuntimeError::new(
                    format!("unsupported unary operator: {other:?}"),
                    expr.span,
                )),
            }
        }
        other => Err(RuntimeError::new(
            format!("unsupported expression: {other:?}"),
            expr.span,
        )),
    }
}

fn var_type_name(vtype: &VarType) -> &'static str {
    match vtype {
        VarType::Void => "void",
        VarType::Int => "int",
        VarType::Float => "float",
        VarType::Str => "str",
        VarType::Bool => "bool",
        VarType::Type => "type",
    }
}

fn var_type_check(vtype: &VarType, value: &Value) -> bool {
    matches!(
        (vtype, value),
        (VarType::Int, Value::Int(_))
            | (VarType::Float, Value::Float(_))
            | (VarType::Bool, Value::Bool(_))
    )
}
