use crate::ast::{self, WhileStatement};

use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> WhileStatement {
    cursor.goto_first_child();

    skip_keywords(cursor);
    let condition = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let body = parse_block(cursor, code);

    cursor.goto_parent();
    WhileStatement { condition, body }
}

pub fn transpile(while_statement: &WhileStatement) -> String {
    let condition = expressions::transpile(&while_statement.condition);
    let body = ast::transpile(&while_statement.body);
    format!("while {condition} {{\n{body}\n}}")
}

fn skip_keywords(cursor: &mut TreeCursor) {
    let mut token = cursor.node().kind();
    while token == "while" {
        if !cursor.goto_next_sibling() {
            break;
        }
        token = cursor.node().kind();
    }
}
