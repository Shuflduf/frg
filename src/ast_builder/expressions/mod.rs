use super::*;

mod binary_operation;
mod function_call;
mod function_literal;
mod index_access;
mod literal;
mod unary_operation;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Expression {
    // cursor.goto_first_child();

    let expression_name = cursor.node().kind();

    println!("{expression_name}");
    let expr = match expression_name {
        "identifier" => Expression::Identifier(code[cursor.node().byte_range()].to_string()),
        "binary_expression" => Expression::BinaryOperation(binary_operation::parse(cursor, code)),
        "unary_expression" => Expression::UnaryOperation(unary_operation::parse(cursor, code)),
        "int_literal" | "string_literal" | "float_literal" | "bool_literal" => {
            Expression::Literal(literal::parse(cursor, code))
        }
        "function_literal" => Expression::FunctionLiteral(function_literal::parse(cursor, code)),
        "function_call" => Expression::FunctionCall(function_call::parse(cursor, code)),
        "index_access" => Expression::IndexAccess(index_access::parse(cursor, code)),
        _ => todo!(),
    };

    // cursor.goto_parent();
    expr
    // todo!()
}
