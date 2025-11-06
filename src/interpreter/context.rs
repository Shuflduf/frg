use std::cell::RefCell;

use super::*;

#[derive(Debug, Clone)]
pub struct VariableData<'a> {
    #[allow(dead_code)]
    pub var_type: &'a VarType,
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
pub struct FunctionData<'a> {
    pub ctx: ExecutionContext<'a>,
    pub ast: &'a Vec<Statement>,
    pub params: &'a Vec<Parameter>,
    #[allow(dead_code)]
    pub return_type: &'a VarType,
}

#[derive(Debug, Default, Clone)]
pub struct ExecutionContext<'a> {
    pub declared_variables: HashMap<String, RefCell<VariableData<'a>>>,
    pub declared_functions: HashMap<String, RefCell<FunctionData<'a>>>,
    // maybe use a tuple struct for this
    pub declared_structs: HashMap<String, HashMap<String, VarType>>,
    pub continue_to_next_if: bool,
}
