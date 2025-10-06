enum ASTNode {
    Module(Vec<ASTNode>),
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
    ExpressionStatement(Box<ASTNode>),
}

enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
}

enum UnaryOp {
    Reference,
    Minus,
    Not,
}

enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryOperation {
        left: Box<ASTNode>,
        op: BinaryOp,
        right: Box<ASTNode>,
    },
    UnaryOperation {
        op: UnaryOp,
        expr: Box<ASTNode>,
    },
    FunctionCall {
        name: String,
        args: Vec<ASTNode>,
    },
}

enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Bool(bool),
}

enum VarType {
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
