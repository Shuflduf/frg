use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> IfStatement {
    cursor.goto_first_child();
    skip_keywords(cursor);
    let condition = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let body = parse_block(cursor, code);

    let mut else_ifs = vec![];
    let mut else_body = vec![];
    while cursor.goto_next_sibling() {
        let node_name = cursor.node().kind();
        match node_name {
            "else_if_statement" => else_ifs.push(parse_else_if(cursor, code)),
            "else_statement" => {
                else_body = parse_else(cursor, code);
                break;
            }
            _ => todo!(),
        }
    }

    cursor.goto_parent();
    IfStatement {
        condition,
        body,
        else_ifs,
        else_body,
    }
}

fn skip_keywords(cursor: &mut TreeCursor) {
    let mut token = cursor.node().kind();
    while token == "if" || token == "else" {
        if !cursor.goto_next_sibling() {
            break;
        };
        token = cursor.node().kind();
    }
}

fn parse_else_if(cursor: &mut TreeCursor, code: &str) -> (Expression, Vec<Statement>) {
    cursor.goto_first_child();
    skip_keywords(cursor);
    let condition = expressions::parse(cursor, code);

    cursor.goto_next_sibling();
    let body = parse_block(cursor, code);

    cursor.goto_parent();
    (condition, body)
}

fn parse_else(cursor: &mut TreeCursor, code: &str) -> Vec<Statement> {
    cursor.goto_first_child();
    skip_keywords(cursor);
    let body = parse_block(cursor, code);

    cursor.goto_parent();
    body
}
