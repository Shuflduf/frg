use std::fmt::Display;

use super::*;

#[derive(Debug, Clone)]
pub struct FunctionValue<'ast> {
    pub params: Vec<&'ast str>,
    pub body: Vec<Stmt<'ast>>,
}

#[derive(Debug, Clone)]
pub enum Value<'ast> {
    Undefined,
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
    Type(TypeValue<'ast>),
    Struct(HashMap<String, Variable<'ast>>),
    List(Vec<Value<'ast>>),
    Function(FunctionValue<'ast>),
    Reference(SlotRef<'ast>),
    // Module()
}

impl Value<'_> {
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Undefined => "undefined",
            Self::Int(_) => "int",
            Self::Float(_) => "float",
            Self::Bool(_) => "bool",
            Self::String(_) => "str",
            Self::Type(_) => "type",
            Self::Struct(_) => "struct",
            Self::List(_) => "list",
            Self::Function(_) => "function",
            Self::Reference(_) => "reference",
        }
    }
    pub fn add(self, rhs: Self) -> Result<Self, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(Self::Int(a + b)),
            (Self::Float(a), Self::Float(b)) => Ok(Self::Float(a + b)),
            (Self::Int(a), Self::Float(b)) => Ok(Self::Float(a as f64 + b)),
            (Self::Float(a), Self::Int(b)) => Ok(Self::Float(a + b as f64)),
            (Self::String(a), Self::String(b)) => Ok(Self::String(a + &b)),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "+",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
    pub fn sub(self, rhs: Self) -> Result<Self, OpError> {
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
    pub fn mul(self, rhs: Self) -> Result<Self, OpError> {
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
    pub fn div(self, rhs: Self) -> Result<Self, OpError> {
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
    pub fn neg(self) -> Result<Self, OpError> {
        match self {
            Self::Int(n) => Ok(Self::Int(-n)),
            Self::Float(n) => Ok(Self::Float(-n)),
            other => Err(OpError::InvalidUnaryOp {
                op: "-",
                expr: other.type_name(),
            }),
        }
    }

    pub fn equals(&self, rhs: &Self) -> Result<bool, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(a == b),
            (Self::Float(a), Self::Float(b)) => Ok(a == b),
            (Self::Int(a), Self::Float(b)) => Ok(*a as f64 == *b),
            (Self::Float(a), Self::Int(b)) => Ok(*a == *b as f64),
            (Self::Bool(a), Self::Bool(b)) => Ok(a == b),
            (Self::String(a), Self::String(b)) => Ok(a == b),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op: "==",
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }

    pub fn greater(&self, rhs: &Self, op: &'static str) -> Result<bool, OpError> {
        match (self, rhs) {
            (Self::Int(a), Self::Int(b)) => Ok(a > b),
            (Self::Float(a), Self::Float(b)) => Ok(a > b),
            (Self::Int(a), Self::Float(b)) => Ok(*a as f64 > *b),
            (Self::Float(a), Self::Int(b)) => Ok(*a > *b as f64),
            (left, right) => Err(OpError::InvalidBinaryOp {
                op,
                left: left.type_name(),
                right: right.type_name(),
            }),
        }
    }
}

impl Display for TypeValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Primitive(t) => f.write_str(&format!("{t:?}")),
            _ => todo!(),
        }
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Undefined => f.write_str("undefined"),
            Self::Int(n) => f.write_str(&n.to_string()),
            Self::Float(n) => f.write_str(&n.to_string()),
            Self::Bool(b) => f.write_str(&b.to_string()),
            Self::String(s) => f.write_str(&s),
            Self::Type(t) => f.write_str(&t.to_string()),
            Self::Struct(s) => f.write_str(&format!(
                "{{ {} }}",
                s.iter()
                    .map(|(k, v)| (k, v.value().to_string()))
                    .map(|(k, v)| format!("{k} = {v}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            Self::List(l) => f.write_str(&format!(
                "[{}]",
                l.iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )),
            _ => todo!(),
        }
    }
}
