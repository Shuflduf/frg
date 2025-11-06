use std::cell::RefCell;

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
pub struct FunctionData {
    pub ctx: ExecutionContext,
    pub ast: Vec<Statement>,
    pub params: Vec<Parameter>,
    #[allow(dead_code)]
    pub return_type: VarType,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionContext {
    pub declared_variables: HashMap<String, RefCell<VariableData>>,
    pub declared_functions: HashMap<String, RefCell<FunctionData>>,
    // maybe use a tuple struct for this
    pub declared_structs: HashMap<String, HashMap<String, VarType>>,
    pub continue_to_next_if: bool,
}
