#[derive(Debug)]
#[allow(dead_code)]
pub enum VarType {
    Void,
    Int,
    Float,
    Str,
    Bool,
    StructDec,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulus,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum UnaryOperator {
    Reference,
    Dereference,
    Negative,
    Not,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Expression<'src> {
    Identifier(&'src str),
    Literal(Literal),
    BinaryOperation {
        left: Box<Expression<'src>>,
        op: BinaryOperator,
        right: Box<Expression<'src>>,
    },
    UnaryOperation {
        op: UnaryOperator,
        expr: Box<Expression<'src>>,
    },
    IndexAccess {
        expr: Box<Expression<'src>>,
        index: Box<Expression<'src>>,
    },
    MemberAccess {
        expr: Box<Expression<'src>>,
        member: &'src str,
    },
    FunctionLiteral {
        params: Vec<&'src str>,
        body: Vec<Statement<'src>>,
    },
    FunctionCall {
        function: Box<Expression<'src>>,
        params: Vec<Expression<'src>>,
    },
    BuiltinCall {
        name: &'src str,
        params: Vec<Expression<'src>>,
    },
    MapLiteral(Vec<(&'src str, Expression<'src>)>),
    VecLiteral(Vec<Expression<'src>>),
    SetLiteral(Vec<Expression<'src>>),
    EmptyCollection,
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Statement<'src> {
    VariableDeclaration {
        vtype: VarType,
        value: Expression<'src>,
    },
}
