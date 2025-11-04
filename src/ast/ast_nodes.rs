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
    #[allow(dead_code)]
    Expression(Expression),
    /// return var
    Return(Expression),
    /// existing_var = 5
    #[allow(dead_code)]
    Assignment { name: String, value: Expression },
    /// if true {}
    Conditional {
        conditional_type: ConditionalType,
        body: Vec<Statement>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConditionalType {
    If(Expression),
    Elif(Expression),
    Else,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    /// \&
    #[allow(dead_code)]
    Reference,
    /// \*
    #[allow(dead_code)]
    Dereference,
    /// \-
    #[allow(dead_code)]
    Negative,
    /// !
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    UnaryOperation { op: UnaryOp, expr: Box<Expression> },
    /// call()
    FunctionCall { name: String, args: Vec<Expression> },
    /// thing.component
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    Set(Vec<Expression>),
    /// { "eggs": 4, "days": 82 }
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    Reference(Box<VarType>),
    Struct(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: String,
    pub param_type: VarType,
}
