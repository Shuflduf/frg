use tree_sitter::{Tree, TreeCursor};

mod expressions;
mod statements;
mod types;

#[derive(Debug)]
pub enum Literal {
    Int(i32),
    Str(String),
}

#[derive(Debug)]
pub enum VarType {
    Int,
    Str,
    StructDec,
    Struct(String),
    Reference(Box<VarType>),
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
pub enum Expression {
    Identifier(String),
    Literal(Literal),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Dereference(Box<Expression>),
    MemberAccess {
        expr: Box<Expression>,
        identifier: String,
    },
}

#[derive(Debug)]
pub struct VariableDeclaration {
    var_type: VarType,
    identifier: String,
    value: Expression,
}

#[derive(Debug)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
}

pub fn build(ts_tree: &Tree, code: &str) -> Vec<Statement> {
    let mut statements = vec![];
    let mut tree_pos = ts_tree.walk();
    tree_pos.goto_first_child();
    // for ts_statement in tree_pos.node(
    // while let current_node = tree_pos.node() {}
    loop {
        let current_node_kind = tree_pos.node().kind();
        statements.push(match current_node_kind {
            "variable_declaration" => statements::parse(&mut tree_pos, code, current_node_kind),
            _ => todo!(),
        });

        // println!("{current_node}");

        if !tree_pos.goto_next_sibling() {
            println!("k im done");
            break;
        }
    }
    statements
}
