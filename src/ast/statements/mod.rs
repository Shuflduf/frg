use tree_sitter::TreeCursor;

use super::*;

mod if_statement;
mod return_statement;
mod variable_declaration;

pub fn parse(cursor: &mut TreeCursor, code: &str, statement_name: &str) -> Statement {
    match statement_name {
        "variable_declaration" => {
            Statement::VariableDeclaration(variable_declaration::parse(cursor, code))
        }
        "if_statement" => Statement::IfStatement(if_statement::parse(cursor, code)),
        "return_statement" => Statement::ReturnStatement(return_statement::parse(cursor, code)),
        _ => todo!(),
    }
}

pub fn transpile(statement: &Statement) -> String {
    match statement {
        Statement::VariableDeclaration(var_decl) => variable_declaration::transpile(var_decl),
        Statement::IfStatement(if_state) => if_statement::transpile(if_state),
        Statement::ReturnStatement(expr) => return_statement::transpile(expr),
        // "if_statement" => Statement::IfStatement(if_statement::parse(cursor, code)),
        // "return_statement" => Statement::ReturnStatement(return_statement::parse(cursor, code)),
        _ => todo!(),
    }
}
