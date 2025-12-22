use crate::ast::{Statement, statements};

pub fn transpile(ast: &Vec<Statement>) -> String {
    let mut code = String::new();
    for statement in ast {
        let new_line = statements::transpile(statement);
        code.push('\n');
        code.push_str(&new_line);
    }
    code
}
