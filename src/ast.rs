use chumsky::span::Spanned;

pub type Expr<'src> = Spanned<Expression<'src>>;
pub type Stmt<'src> = Spanned<Statement<'src>>;

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum VarType<'src> {
    Void,
    Int,
    Float,
    Str,
    Bool,
    Type,
    Custom(Expr<'src>),
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum UnaryOperator {
    Reference,
    Dereference,
    Negative,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum AssignmentOperator {
    Equals,
    PlusEquals,
    MinusEquals,
    TimesEquals,
    DivideEquals,
    ModulusEquals,
}

#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
pub enum Expression<'src> {
    Identifier(&'src str),
    Literal(Literal),
    BinaryOperation {
        left: Box<Expr<'src>>,
        op: BinaryOperator,
        right: Box<Expr<'src>>,
    },
    UnaryOperation {
        op: UnaryOperator,
        right: Box<Expr<'src>>,
    },
    IndexAccess {
        parent: Box<Expr<'src>>,
        index: Box<Expr<'src>>,
    },
    MemberAccess {
        parent: Box<Expr<'src>>,
        member: Spanned<&'src str>,
    },
    FunctionLiteral {
        params: Vec<&'src str>,
        body: Vec<Stmt<'src>>,
    },
    FunctionCall {
        function: Box<Expr<'src>>,
        params: Vec<Expr<'src>>,
    },
    // BuiltinCall {
    //     name: &'src str,
    //     params: Vec<Expr<'src>>,
    // },
    MapLiteral(Vec<(&'src str, Expr<'src>)>),
    VecLiteral(Vec<Expr<'src>>),
    SetLiteral(Vec<Expr<'src>>),
    EmptyCollection,

    StructTypeDeclaration {
        fields: Vec<(VarType<'src>, Spanned<&'src str>, &'src str)>,
    },
    StructLiteral {
        parent: Box<Expr<'src>>,
        fields: Vec<(&'src str, Expr<'src>)>,
    },
}

#[derive(Debug, PartialEq, Clone)]
#[allow(dead_code)]
pub enum Statement<'src> {
    VariableDeclaration {
        vtype_name: Spanned<&'src str>,
        vtype: VarType<'src>,
        name: Spanned<&'src str>,
        value: Option<Expr<'src>>,
    },
    IfStatement {
        condition: Expr<'src>,
        body: Vec<Stmt<'src>>,
        else_ifs: Vec<(Expr<'src>, Vec<Stmt<'src>>)>,
        else_body: Vec<Stmt<'src>>,
    },
    VariableAssignment {
        variable: Expr<'src>,
        op: AssignmentOperator,
        expr: Expr<'src>,
    },
}
