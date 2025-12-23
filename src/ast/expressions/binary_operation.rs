use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> BinaryOperation {
    cursor.goto_first_child();
    let left = Box::new(expressions::parse(cursor, code));

    cursor.goto_next_sibling();
    let op_symbol = cursor.node().kind();
    let op = match op_symbol {
        "+" => BinaryOperator::Add,
        "-" => BinaryOperator::Subtract,
        "*" => BinaryOperator::Multiply,
        "/" => BinaryOperator::Divide,
        "<" => BinaryOperator::LessThan,
        ">" => BinaryOperator::GreaterThan,
        "<=" => BinaryOperator::LessThanOrEqual,
        ">=" => BinaryOperator::GreaterThanOrEqual,
        "==" => BinaryOperator::Equals,
        "!=" => BinaryOperator::NotEquals,
        _ => unreachable!(),
    };

    cursor.goto_next_sibling();
    let right = Box::new(expressions::parse(cursor, code));
    // let op = match cursor.nod

    cursor.goto_parent();
    BinaryOperation { left, op, right }
    // todo!()
}

pub fn transpile(binary_op: &BinaryOperation) -> String {
    let left = expressions::transpile(&binary_op.left);
    let right = expressions::transpile(&binary_op.right);
    let op = match binary_op.op {
        BinaryOperator::Add => "+",
        BinaryOperator::Subtract => "-",
        BinaryOperator::Multiply => "*",
        BinaryOperator::Divide => "/",
        BinaryOperator::LessThan => "<",
        BinaryOperator::LessThanOrEqual => "<=",
        BinaryOperator::GreaterThan => ">",
        BinaryOperator::GreaterThanOrEqual => ">=",
        BinaryOperator::Equals => "==",
        BinaryOperator::NotEquals => "!=",
    };
    return format!("{left} {op} {right}");
}
