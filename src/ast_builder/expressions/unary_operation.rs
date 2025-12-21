use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> UnaryOperation {
    cursor.goto_first_child();
    let op_symbol = cursor.node().kind();

    cursor.goto_next_sibling();
    let op = match op_symbol {
        "-" => UnaryOperator::Negative,
        "&" => UnaryOperator::Reference,
        "!" => UnaryOperator::Not,
        _ => unreachable!(),
    };
    let expr = Box::new(expressions::parse(cursor, code));

    UnaryOperation { op, expr }
}
