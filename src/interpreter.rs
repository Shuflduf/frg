use std::collections::HashMap;

use chumsky::span::SimpleSpan;

use crate::{
    ast::{BinaryOperator, Expr, Expression, Literal, Statement, Stmt, UnaryOperator, VarType},
    errors::{Diagnostic, OpError},
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
    fn sub(self, rhs: Self) -> Result<Self, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a - b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a - b)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 - b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a - b as f64)),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "-",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
    fn mul(self, rhs: Self) -> Result<Self, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a * b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a * b)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 * b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a * b as f64)),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "*",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
    fn div(self, rhs: Self) -> Result<Self, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => {
                if b == 0 {
                    Err(OpError::DivisionByZero)
                } else {
                    Ok(Self::Int(a / b))
                }
            }
            (Self::Float(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(OpError::DivisionByZero)
                } else {
                    Ok(Self::Float(a / b))
                }
            }
            (Self::Int(a), Self::Float(b)) => {
                if b == 0.0 {
                    Err(OpError::DivisionByZero)
                } else {
                    Ok(Self::Float(a as f64 / b))
                }
            }
            (Self::Float(a), Self::Int(b)) => {
                if b == 0 {
                    Err(OpError::DivisionByZero)
                } else {
                    Ok(Self::Float(a / b as f64))
                }
            }
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "/",
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

#[derive(Debug)]
struct State<'src> {
    variables: HashMap<&'src str, Value>,
}

pub fn run(program: &[Stmt]) -> Result<i64, Diagnostic> {
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
                    return Err(Diagnostic::new(format!(
                        "variable `{}` expects `{expected}`, got `{found}`",
                        name.inner
                    ))
                    .with_label(format!("declared as `{expected}`"), name.span)
                    .with_label(format!("this is `{found}`"), value.span));
                }
                state.variables.insert(name.inner, val);
            }
            other => {
                return Err(Diagnostic::basic(
                    format!("unsupported statement: {other:?}"),
                    stmt.span,
                ));
            }
        }
    }

    println!("{state:?}");

    Ok(0)
}

fn eval(state: &State, expr: &Expr) -> Result<Value, Diagnostic> {
    match &expr.inner {
        Expression::Identifier(name) => state
            .variables
            .get(name)
            .cloned()
            .ok_or_else(|| Diagnostic::basic(format!("undefined variable `{name}`"), expr.span)),
        Expression::Literal(lit) => match lit {
            Literal::Int(n) => Ok(Value::Int(*n)),
            Literal::Float(n) => Ok(Value::Float(*n)),
            Literal::Bool(b) => Ok(Value::Bool(*b)),
            other => Err(Diagnostic::basic(
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
                    .map_err(|e| Diagnostic::basic(e.to_string(), expr.span)),
                BinaryOperator::Subtract => left
                    .sub(right)
                    .map_err(|e| Diagnostic::basic(e.to_string(), expr.span)),
                BinaryOperator::Multiply => left
                    .mul(right)
                    .map_err(|e| Diagnostic::basic(e.to_string(), expr.span)),
                BinaryOperator::Divide => left
                    .div(right)
                    .map_err(|e| Diagnostic::basic(e.to_string(), expr.span)),
                BinaryOperator::Equals => {
                    Ok(Value::Bool(left.equals(&right).map_err(|e| {
                        Diagnostic::basic(e.to_string(), expr.span)
                    })?))
                }
                BinaryOperator::NotEquals => {
                    let equal = left
                        .equals(&right)
                        .map_err(|e| Diagnostic::basic(e.to_string(), expr.span))?;
                    Ok(Value::Bool(!equal))
                }
                other => Err(Diagnostic::basic(
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
                    .map_err(|e| Diagnostic::basic(e.to_string(), expr.span)),
                other => Err(Diagnostic::basic(
                    format!("unsupported unary operator: {other:?}"),
                    expr.span,
                )),
            }
        }
        other => Err(Diagnostic::basic(
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

// fn Diagnostic::basic(message: impl Into<String>, span: SimpleSpan) -> Diagnostic {
//     Diagnostic::new(message.into().clone()).with_label(message, span)
// }
