use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> BinaryOperation {
    cursor.goto_first_child();
    let left = Box::new(expressions::parse(cursor, code));

    cursor.goto_next_sibling();
    let op_symbol = cursor.node().kind();
    let op = match op_symbol {
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        _ => todo!(),
    };

    cursor.goto_next_sibling();
    let right = Box::new(expressions::parse(cursor, code));
    // let op = match cursor.nod

    cursor.goto_parent();
    BinaryOperation { left, op, right }
    // todo!()
}
