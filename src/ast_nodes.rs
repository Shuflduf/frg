enum ASTNode {
    Program(Vec<ASTNode>),
    Statement(Statement),
    Expression(Expression),
}

enum Statement {
    VariableDeclaration {
        var_type: VarType,
        name: String,
        value: Box<ASTNode>,
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
    ExpressionStatement(Box<Expression>),
}

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum UnaryOp {
    Reference,
    Dereference,
    Negative,
    Not,
}

enum Expression {
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

enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

enum CompositeLiteral {
    Vec(Vec<Expression>),
    Set(Vec<Expression>),
    Map(Vec<(Expression, Expression)>),
    Struct(Vec<(String, Expression)>),
}

enum VarType {
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

struct Parameter {
    name: String,
    param_type: VarType,
}
