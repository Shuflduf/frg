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

    cursor.goto_parent();

    UnaryOperation { op, expr }
}

pub fn transpile(unary_op: &UnaryOperation) -> String {
    let op = match unary_op.op {
        UnaryOperator::Reference => "&mut ",
        UnaryOperator::Negative => "-",
        UnaryOperator::Not => "!",
    };
    let expr = expressions::transpile(&unary_op.expr);
    format!("{op}{expr}")
}
