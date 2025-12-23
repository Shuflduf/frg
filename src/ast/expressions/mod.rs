use super::*;

mod binary_operation;
mod builtin;
mod dereference;
mod function_call;
mod function_literal;
mod index_access;
mod literal;
mod map_literal;
mod member_access;
mod unary_operation;
mod vec_literal;

pub fn parse(cursor: &mut TreeCursor, code: &str) -> Expression {
    // cursor.goto_first_child();

    let expression_name = cursor.node().kind();

    // cursor.goto_parent();
    match expression_name {
        "int_literal" | "string_literal" | "float_literal" | "bool_literal" => {
            Expression::Literal(literal::parse(cursor, code))
        }
        "identifier" => Expression::Identifier(code[cursor.node().byte_range()].to_string()),
        "binary_expression" => Expression::BinaryOperation(binary_operation::parse(cursor, code)),
        "unary_expression" => Expression::UnaryOperation(unary_operation::parse(cursor, code)),
        "function_literal" => Expression::FunctionLiteral(function_literal::parse(cursor, code)),
        "function_call" => Expression::FunctionCall(function_call::parse(cursor, code)),
        "index_access" => Expression::IndexAccess(index_access::parse(cursor, code)),
        "builtin" => Expression::BuiltinCall(builtin::parse(cursor, code)),
        "map_literal" => Expression::MapLiteral(map_literal::parse(cursor, code)),
        "vec_literal" => Expression::VecLiteral(vec_literal::parse(cursor, code)),
        "dereference" => Expression::Dereference(dereference::parse(cursor, code)),
        "member_access" => Expression::MemberAccess(member_access::parse(cursor, code)),
        _ => todo!("{expression_name}"),
    }
    // todo!()
}

pub fn transpile(expr: &Expression) -> String {
    match expr {
        Expression::Literal(lit) => literal::transpile(lit),
        Expression::Identifier(identifier) => identifier.clone(),
        Expression::BinaryOperation(binary_op) => binary_operation::transpile(binary_op),
        Expression::FunctionLiteral(lit) => function_literal::transpile(lit),
        Expression::FunctionCall(func_call) => function_call::transpile(func_call),
        Expression::BuiltinCall(builtin_call) => builtin::transpile(builtin_call),
        Expression::MapLiteral(map_lit) => map_literal::transpile(map_lit),
        Expression::VecLiteral(vec_lit) => vec_literal::transpile(vec_lit),
        Expression::Dereference(inner) => dereference::transpile(inner),
        Expression::UnaryOperation(unary_op) => unary_operation::transpile(unary_op),
        Expression::MemberAccess(mem_acc) => member_access::transpile(mem_acc),
        _ => todo!("{expr:?}"),
    }
}

pub fn transpile_list(values: &Vec<String>) -> String {
    let mut list = String::new();
    for value in values {
        list.push_str(value);
        list.push_str(", ");
    }
    list.pop();
    list.pop();
    list
}
