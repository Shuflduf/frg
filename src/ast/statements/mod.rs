use tree_sitter::TreeCursor;

use super::{
    AssignmentOp, Expression, IfStatement, Statement, VarType, VariableDeclaration, expressions,
    parse_block, skip_repeats, types,
};

mod if_statement;
mod return_statement;
mod struct_declaration;
mod variable_assignment;
mod variable_declaration;
mod void_statement;
pub mod while_statement;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Statement {
    cursor.goto_first_child();

    let statement_name = cursor.node().kind();
    let statement = match statement_name {
        "variable_declaration" => {
            Statement::VariableDeclaration(variable_declaration::parse(cursor, code))
        }
        "if_statement" => Statement::IfStatement(if_statement::parse(cursor, code)),
        "return_statement" => Statement::ReturnStatement(return_statement::parse(cursor, code)),
        "struct_declaration" => {
            Statement::StructDeclaration(struct_declaration::parse(cursor, code))
        }
        "variable_assignment" => {
            Statement::VariableAssignment(variable_assignment::parse(cursor, code))
        }
        "expression" => Statement::Expression(expressions::parse(cursor, code)),
        "void_statement" => Statement::VoidStatement(void_statement::parse(cursor, code)),
        "while_statement" => Statement::WhileStatement(while_statement::parse(cursor, code)),
        _ => todo!("{statement_name}"),
    };
    cursor.goto_parent();

    statement
}

pub fn transpile(statement: &Statement) -> String {
    match statement {
        Statement::VariableDeclaration(var_decl) => variable_declaration::transpile(var_decl),
        Statement::IfStatement(if_state) => if_statement::transpile(if_state),
        Statement::ReturnStatement(expr) => return_statement::transpile(expr),
        Statement::Expression(expr) => format!("{};", expressions::transpile(expr)),
        Statement::StructDeclaration(struct_decl) => struct_declaration::transpile(struct_decl),
        Statement::VariableAssignment(var_ass) => variable_assignment::transpile(var_ass),
        Statement::VoidStatement(expr) => void_statement::transpile(expr),
        Statement::WhileStatement(while_statement) => while_statement::transpile(while_statement),
    }
}
