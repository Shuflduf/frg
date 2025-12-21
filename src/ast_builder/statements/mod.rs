use tree_sitter::TreeCursor;

use super::*;

mod variable_declaration;

pub fn parse(cursor: &mut TreeCursor, code: &str, statement_name: &str) -> Statement {
    match statement_name {
        "variable_declaration" => {
            Statement::VariableDeclaration(variable_declaration::parse(cursor, code))
        }
        _ => todo!(),
    }
}
