use super::*;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> BinaryOperation {
    cursor.goto_first_child();
    println!("code check 1 {}", &code[cursor.node().byte_range()]);
    let left = Box::new(expressions::parse(cursor, code));

    println!("left {left:?}");
    cursor.goto_next_sibling();
    println!("code check 2 {}", &code[cursor.node().byte_range()]);
    let op_symbol = cursor.node().kind();
    println!("huh {op_symbol:?}");
    // println!("huh {:?}", cursor.node().range());
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
    println!("{left:?} {op:?} {right:?}");
    BinaryOperation { left, op, right }
    // todo!()
}
