#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum VarType {
    Void,
    Int,
    Float,
    Str,
    StructDec,
    Struct(String),
    Reference(Box<VarType>),
    Function {
        return_type: Box<VarType>,
        param_types: Vec<VarType>,
    },
    Vec(Box<VarType>),
    Set(Box<VarType>),
    Map((Box<VarType>, Box<VarType>)),
}

#[derive(Debug)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    Equals,
    NotEquals,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Reference,
    Negative,
    Not,
}

#[derive(Debug)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub op: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryOperation {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
}

#[derive(Debug)]
pub struct IndexAccess {
    pub expr: Box<Expression>,
    pub target_index: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionLiteral {
    pub params: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionCall {
    pub function: Box<Expression>,
    pub params: Vec<Expression>,
}

#[derive(Debug)]
pub struct BuiltinCall {
    pub name: String,
    pub params: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    IndexAccess(IndexAccess),
    Dereference(Box<Expression>),
    MemberAccess((Box<Expression>, String)),
    FunctionLiteral(FunctionLiteral),
    FunctionCall(FunctionCall),
    BuiltinCall(BuiltinCall),
    MapLiteral(Vec<(String, Expression)>),
    VecLiteral(Vec<Expression>),
    SetLiteral(Vec<Expression>),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    pub var_type: VarType,
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    pub condition: Expression,
    pub body: Vec<Statement>,
    pub else_ifs: Vec<(Expression, Vec<Statement>)>,
    pub else_body: Vec<Statement>,
}

#[derive(Debug)]
pub struct StructDeclaration {
    pub name: String,
    pub fields: (String, VarType),
}

#[derive(Debug)]
pub enum AssignmentOp {
    Equals,
    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    IfStatement(IfStatement),
    ReturnStatement(Expression),
    StructDeclaration((String, Vec<(VarType, String)>)),
    VariableAssignment((Expression, AssignmentOp, Expression)),
}
