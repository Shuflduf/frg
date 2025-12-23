use crate::ast::{self, Statement};

pub fn transpile(statements: &Vec<Statement>) -> String {
    let code = ast::transpile(statements);
    format!("fn main() {{\n{code}\n}}")
}
