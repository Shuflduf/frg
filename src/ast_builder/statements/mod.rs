use tree_sitter::TreeCursor;

use super::*;

mod if_statement;
mod variable_declaration;

pub fn parse(cursor: &mut TreeCursor, code: &str, statement_name: &str) -> Statement {
    match statement_name {
        "variable_declaration" => {
            Statement::VariableDeclaration(variable_declaration::parse(cursor, code))
        }
        "if_statement" => Statement::IfStatement(if_statement::parse(cursor, code)),
        _ => todo!(),
    }
}
