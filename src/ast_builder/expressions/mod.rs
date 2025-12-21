use super::*;

mod binary_operation;
mod literal;
mod unary_operation;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Expression {
    // cursor.goto_first_child();

    let expression_name = cursor.node().kind();
    println!("{}", expression_name);

    match expression_name {
        "binary_expression" => Expression::BinaryOperation(binary_operation::parse(cursor, code)),
        "unary_expression" => Expression::UnaryOperation(unary_operation::parse(cursor, code)),
        "number_literal" => Expression::Literal(literal::parse(cursor, code)),
        _ => todo!(),
    }

    // cursor.goto_parent();
    // todo!()
}
