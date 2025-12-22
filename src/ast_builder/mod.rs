use tree_sitter::{Tree, TreeCursor};

mod expressions;
mod statements;
mod types;

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
    Index,
}

#[derive(Debug)]
pub enum UnaryOperator {
    Reference,
    Negative,
    Not,
}

#[derive(Debug)]
pub struct BinaryOperation {
    left: Box<Expression>,
    op: BinaryOperator,
    right: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryOperation {
    op: UnaryOperator,
    expr: Box<Expression>,
}

#[derive(Debug)]
pub struct IndexAccess {
    expr: Box<Expression>,
    target_index: Box<Expression>,
}

#[derive(Debug)]
pub struct FunctionLiteral {
    params: Vec<String>,
    body: Vec<Statement>,
}

#[derive(Debug)]
pub struct FunctionCall {
    function: Box<Expression>,
    params: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    IndexAccess(IndexAccess),
    Dereference(Box<Expression>),
    MemberAccess {
        expr: Box<Expression>,
        identifier: String,
    },
    FunctionLiteral(FunctionLiteral),
    FunctionCall(FunctionCall),
}

#[derive(Debug)]
pub struct VariableDeclaration {
    var_type: VarType,
    identifier: String,
    value: Expression,
}

#[derive(Debug)]
pub struct IfStatement {
    condition: Expression,
    body: Vec<Statement>,
    else_if_body: Vec<(Expression, Vec<Statement>)>,
    else_body: Option<Box<Statement>>,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Expression(Expression),
    IfStatement(IfStatement),
}

pub fn build(ts_tree: &Tree, code: &str) -> Vec<Statement> {
    let mut cursor = ts_tree.walk();
    parse_block(&mut cursor, code)
}

fn parse_block(cursor: &mut TreeCursor, code: &str) -> Vec<Statement> {
    let mut statements = vec![];
    cursor.goto_first_child();
    // for ts_statement in tree_pos.node(
    // while let current_node = tree_pos.node() {}
    loop {
        let current_node_kind = cursor.node().kind();
        // println!("{current_node_kind}");
        statements.push(match current_node_kind {
            "{" => {
                cursor.goto_next_sibling();
                continue;
            }
            "variable_declaration" | "if_statement" => {
                statements::parse(cursor, code, current_node_kind)
            }
            "function_call" => Statement::Expression(expressions::parse(cursor, code)),
            "}" => break,
            // "block"
            _ => todo!("{current_node_kind}"),
        });

        // println!("{current_node}");

        if !cursor.goto_next_sibling() {
            println!("k im done");
            break;
        }
    }
    statements
}
