use std::{cell::RefCell, fmt::Display, rc::Rc};

use super::*;

#[derive(Debug, Clone)]
pub struct VariableData {
    #[allow(dead_code)]
    pub var_type: VarType,
    pub value: VariableValue,
}

#[derive(Debug, Clone)]
pub enum VariableValue {
    Void,
    Int(i32),
    Str(String),
    Float(f32),
    Bool(bool),
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
impl Display for VariableValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Void => write!(f, "void"),
            Self::Int(new_int) => write!(f, "{new_int}"),
            Self::Str(new_str) => write!(f, "{new_str}"),
            Self::Float(new_float) => write!(f, "{new_float}"),
            Self::Bool(new_bool) => write!(f, "{new_bool}"),
            Self::Vec(new_vec) => write!(
                f,
                "{:?}",
                new_vec
                    .iter()
                    .map(|elem| elem.to_string())
                    .collect::<Vec<String>>()
            ),
            _ => todo!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionData {
    pub ctx: ExecutionContext,
    pub ast: Vec<Statement>,
    pub params: Vec<Parameter>,
    #[allow(dead_code)]
    pub return_type: VarType,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionContext {
    pub declared_variables: HashMap<String, Rc<RefCell<VariableData>>>,
    pub declared_functions: HashMap<String, Rc<FunctionData>>,
    // maybe use a tuple struct for this
    pub declared_structs: HashMap<String, Rc<HashMap<String, VarType>>>,
    pub continue_to_next_if: bool,
}
