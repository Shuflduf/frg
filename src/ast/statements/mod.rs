use tree_sitter::TreeCursor;

use super::*;

mod if_statement;
mod return_statement;
mod struct_declaration;
mod variable_assignment;
mod variable_declaration;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Statement {
    cursor.goto_first_child();

    println!("1: {}", &code[cursor.node().byte_range()]);
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
    }
}
