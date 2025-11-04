use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    /// int var = 5
    VariableDeclaration {
        var_type: VarType,
        name: String,
        value: Expression,
    },
    /// void func = () {}
    FunctionDeclaration {
        return_type: VarType,
        name: String,
        params: Vec<Parameter>,
        body: Vec<Statement>,
    },
    /// struct Frog {}
    StructDeclaration {
        name: String,
        fields: Vec<Parameter>,
    },
    /// call()
    /// basically throwing away the returned value
    Expression(Expression),
    /// return var
    Return(Expression),
    /// existing_var = 5
    Assignment {
        name: String,
        value: Box<Expression>,
    },
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
    /// \&
    Reference,
    /// \*
    Dereference,
    /// \-
    Negative,
    /// !
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// my_var
    Identifier(String),
    /// "string data"
    Literal(Literal),
    /// [4, 2]
    CompositeLiteral(CompositeLiteral),
    /// 5 + 2
    BinaryOperation {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    /// !true
    UnaryOperation { op: UnaryOp, expr: Box<Expression> },
    /// call()
    FunctionCall { name: String, args: Vec<Expression> },
    /// thing.component
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
    /// [4, 2]
    Vec(Vec<Expression>),
    /// {4, 2}
    /// Unordered with no duplicates
    Set(Vec<Expression>),
    /// { "eggs": 4, "days": 82 }
    Map(Vec<(Expression, Expression)>),
    /// Frog chicken = {}
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
