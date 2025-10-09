use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        var_type: VarType,
        name: String,
        value: Expression,
    },
    FunctionDeclaration {
        return_type: VarType,
        name: String,
        params: Vec<Parameter>,
        body: Vec<ASTNode>,
    },
    StructDeclaration {
        name: String,
        fields: Vec<Parameter>,
    },
    Expression(Box<Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Reference,
    Dereference,
    Negative,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    CompositeLiteral(CompositeLiteral),
    BinaryOperation {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    UnaryOperation {
        op: UnaryOp,
        expr: Box<Expression>,
    },
    FunctionCall {
        name: String,
        args: Vec<Expression>,
    },
    Assignment {
        name: String,
        value: Box<Expression>,
    },
    MemberAccess {
        object: Box<Expression>,
        field: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompositeLiteral {
    Vec(Vec<Expression>),
    Set(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
    Struct(HashMap<String, Expression>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum VarType {
    Void,
    Int,
    Str,
    Float,
    Bool,
    Vec(Box<VarType>),
    Map(Box<VarType>, Box<VarType>),
    Set(Box<VarType>),
    Reference(Box<VarType>),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: VarType,
}
