use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::ast_nodes::*,
    lexer::{self, Token},
};

pub mod ast_nodes;
mod loops;
mod parse_conditional;
mod parse_declaration;
mod parse_function;
mod parse_list;
mod parse_struct;
mod parse_types;

pub fn expect_token(token_iter: &mut Peekable<Iter<Token>>, expected: lexer::Token) {
    match token_iter.next() {
        Some(s) if *s == expected => {}
        _ => panic!("Expected {:?}", expected),
    }
}

pub fn expect_symbol(token_iter: &mut Peekable<Iter<Token>>, expected: lexer::Symbol) {
    expect_token(token_iter, Token::Symbol(expected));
}

pub fn parse_next_statement(token_iter: &mut Peekable<Iter<Token>>) -> Option<Statement> {
    if let Some(token) = token_iter.peek() {
        let new_node = match token {
            Token::Keyword(lexer::Keyword::Struct) => parse_struct::parse_type(token_iter),
            Token::Keyword(lexer::Keyword::Return) => parse_function::parse_return(token_iter),
            Token::Keyword(lexer::Keyword::If)
            | Token::Keyword(lexer::Keyword::Elif)
            | Token::Keyword(lexer::Keyword::Else) => parse_conditional::parse(token_iter),
            Token::Keyword(lexer::Keyword::Loop) => loops::parse(token_iter),
            Token::Keyword(_) => parse_declaration::parse(token_iter),
            // could be a bad idea
            Token::Literal(lexer::Literal::Identifier(_)) => {
                if parse_struct::is_struct_definition(token_iter.clone()) {
                    parse_struct::parse_declaration(token_iter)
                } else {
                    match parse_expression(token_iter) {
                        Expression::FunctionCall { name, args } => {
                            Statement::Expression(Expression::FunctionCall { name, args })
                        }
                        Expression::Identifier(identifier) => {
                            parse_assignment(token_iter, &identifier)
                        }
                        _ => todo!(),
                    }
                    // TODO: more things would go here here attributes and shit
                    // parse_assignment(token_iter)
                }
            }
            _ => return None,
        };
        Some(new_node)
    } else {
        None
    }
}

pub fn parse(tokens: Vec<Token>) -> Vec<Statement> {
    let mut nodes = vec![];
    let mut token_iter = tokens.iter().peekable();
    while let Some(new_node) = parse_next_statement(&mut token_iter) {
        // dbg!(&new_node);
        nodes.push(new_node);
    }
    nodes
}

fn parse_single_value(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(id))) => {
            let mut func_name = id.to_string();
            if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Exclamation)) {
                func_name.push('!');
                token_iter.next();
            }
            if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::LeftParen)) {
                let args = parse_function::parse_arguments(token_iter);
                Expression::FunctionCall {
                    name: func_name,
                    args,
                }
            } else {
                parse_literal(&lexer::Literal::Identifier(id.clone()))
                // Expression::Identifier(id.clone())
            }
        }
        Some(Token::Keyword(keyword)) => {
            let args = parse_function::parse_arguments(token_iter);
            Expression::Conversion {
                to: parse_types::match_lexer_types(keyword),
                expr: Box::new(args[0].clone()),
            }
        }
        Some(Token::Literal(lit)) => parse_literal(lit),
        Some(Token::Symbol(lexer::Symbol::LeftBracket)) => parse_list::parse_vec(token_iter),
        Some(Token::Symbol(lexer::Symbol::LeftBrace)) => parse_list::parse_set_or_map(token_iter),
        Some(Token::Symbol(lexer::Symbol::Minus)) => Expression::UnaryOperation {
            op: UnaryOp::Negative,
            expr: Box::new(parse_single_value(token_iter)),
        },
        Some(Token::Symbol(lexer::Symbol::Exclamation)) => Expression::UnaryOperation {
            op: UnaryOp::Not,
            expr: Box::new(parse_single_value(token_iter)),
        },

        _ => panic!("literal or identifier"),
    }
}

fn parse_assignment(token_iter: &mut Peekable<Iter<Token>>, identifier: &str) -> Statement {
    let op = match token_iter.next() {
        Some(Token::Symbol(lexer::Symbol::Equals)) => AssingmentOp::Equals,
        Some(Token::Symbol(lexer::Symbol::PlusEquals)) => AssingmentOp::PlusEquals,
        Some(Token::Symbol(lexer::Symbol::MinusEquals)) => AssingmentOp::MinusEquals,
        Some(Token::Symbol(lexer::Symbol::StarEquals)) => AssingmentOp::TimesEquals,
        Some(Token::Symbol(lexer::Symbol::FSlashEquals)) => AssingmentOp::DivideEquals,
        _ => panic!("assignment op expected after identifier"),
    };
    let value = parse_expression(token_iter);
    Statement::Assignment {
        name: identifier.into(),
        value,
        op,
    }
}

fn parse_expression(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    let mut expr = parse_single_value(token_iter);
    while let Some(token) = token_iter.peek() {
        let operation = match token {
            Token::Symbol(lexer::Symbol::Plus) => BinaryOp::Add,
            Token::Symbol(lexer::Symbol::Minus) => BinaryOp::Subtract,
            Token::Symbol(lexer::Symbol::Star) => BinaryOp::Multiply,
            Token::Symbol(lexer::Symbol::FSlash) => BinaryOp::Divide,
            Token::Symbol(lexer::Symbol::LessThan) => BinaryOp::LessThan,
            Token::Symbol(lexer::Symbol::LessThanOrEqual) => BinaryOp::LessThanOrEqual,
            Token::Symbol(lexer::Symbol::GreaterThan) => BinaryOp::GreaterThan,
            Token::Symbol(lexer::Symbol::GreaterThanOrEqual) => BinaryOp::GreaterThanOrEqual,
            Token::Symbol(lexer::Symbol::DoubleEquals) => BinaryOp::Equals,
            Token::Symbol(lexer::Symbol::LeftBracket) => BinaryOp::Index,
            _ => break,
        };
        append_operation(token_iter, operation, &mut expr);
    }
    expr
}

fn commas(token_iter: &mut Peekable<Iter<Token>>) {
    while let Some(token) = token_iter.peek() {
        if let Token::Symbol(lexer::Symbol::Comma) = token {
            token_iter.next();
        } else {
            break;
        }
    }
}

fn append_operation(
    token_iter: &mut Peekable<Iter<Token>>,
    operation: BinaryOp,
    expr: &mut Expression,
) {
    token_iter.next();
    let right = parse_single_value(token_iter);
    *expr = Expression::BinaryOperation {
        left: Box::new(expr.clone()),
        op: operation,
        right: Box::new(right),
    };
}

fn parse_literal(literal: &lexer::Literal) -> Expression {
    match literal {
        lexer::Literal::Number(new_num) => {
            if new_num.contains(".") {
                Expression::Literal(Literal::Float(new_num.parse().unwrap()))
            } else {
                Expression::Literal(Literal::Int(new_num.parse().unwrap()))
            }
        }
        lexer::Literal::String(new_str) => {
            Expression::Literal(Literal::String(new_str.to_string()))
        }
        lexer::Literal::Identifier(new_identifier) => {
            if new_identifier.as_str() == "true" {
                Expression::Literal(Literal::Bool(true))
            } else if new_identifier.as_str() == "false" {
                Expression::Literal(Literal::Bool(false))
            } else {
                Expression::Identifier(new_identifier.to_string())
            }
        }
    }
}
