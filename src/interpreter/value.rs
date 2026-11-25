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
    Module(HashMap<String, Variable<'ast>>),
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
            Self::Module(_) => "mod",
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
        let display = match self {
            Self::Primitive(t) => match t {
                VarType::Void => "void",
                VarType::Int => "int",
                VarType::Float => "float",
                VarType::Str => "str",
                VarType::Bool => "bool",
                VarType::Type => "type",
                VarType::Module => "mod",
                VarType::Custom(expr) => &format!("{expr:?}"),
            },
            _ => todo!(),
        };
        f.write_str(display)
    }
}

impl Display for Value<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display = match self {
            Self::Undefined => "undefined",
            Self::Int(n) => &n.to_string(),
            Self::Float(n) => &n.to_string(),
            Self::Bool(b) => &b.to_string(),
            Self::String(s) => &s,
            Self::Type(t) => &t.to_string(),
            Self::Struct(s) => &format!(
                "{{ {} }}",
                s.iter()
                    .map(|(k, v)| (k, v.value().to_string()))
                    .map(|(k, v)| format!("{k} = {v}"))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::List(l) => &format!(
                "[{}]",
                l.iter()
                    .map(|value| value.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
            Self::Function(v) => &format!(
                "|{}| {{ {:?} returns }}",
                v.params.join(", "),
                count_returns(&v.body)
            ),
            Self::Reference(r) => &format!("&{}", r.borrow().vtype),
            Self::Module(s) => &format!(
                "mod {{ {} }}",
                s.iter()
                    .map(|(k, v)| format!("{k} = {}", v.value()))
                    .collect::<Vec<String>>()
                    .join(", ")
            ),
        };
        f.write_str(display)
    }
}

fn count_returns(block: &[Stmt]) -> u32 {
    let mut returns = 0;
    for stmt in block {
        match stmt.inner {
            Statement::Return(_) => returns += 1,
            Statement::If {
                ref body,
                ref else_ifs,
                ref else_body,
                ..
            } => {
                returns += count_returns(&body)
                    + else_ifs
                        .iter()
                        .map(|(_, else_if_body)| count_returns(else_if_body))
                        .sum::<u32>()
                    + count_returns(else_body)
            }
            _ => {}
        }
    }
    returns
}
