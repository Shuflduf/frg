use std::{iter::Peekable, slice::Iter};

use crate::{
    ast::ast_nodes::*,
    lexer::{self, Token},
};

mod ast_nodes;
mod parse_struct;
mod parse_types;
mod parse_vec;

pub fn expect_token(token_iter: &mut Peekable<Iter<Token>>, expected: lexer::Token) {
    match token_iter.next() {
        Some(s) if *s == expected => {}
        _ => panic!("Expected {:?}", expected),
    }
}

pub fn expect_symbol(token_iter: &mut Peekable<Iter<Token>>, expected: lexer::Symbol) {
    expect_token(token_iter, Token::Symbol(expected));
}

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = vec![];
    let mut token_iter = tokens.iter().peekable();
    while let Some(token) = token_iter.peek() {
        let new_node = match token {
            Token::Keyword(lexer::Keyword::Struct) => parse_struct::parse_type(&mut token_iter),
            Token::Keyword(_) => {
                let var_type = parse_types::parse(&mut token_iter);
                let name = match token_iter.next() {
                    Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
                    _ => panic!("identifier after type"),
                };
                expect_symbol(&mut token_iter, lexer::Symbol::Equals);
                let value = parse_expression(&mut token_iter);
                ASTNode::Statement(Statement::VariableDeclaration {
                    var_type,
                    name: name.clone(),
                    value,
                })
            }
            // could be a bad idea
            Token::Literal(lexer::Literal::Identifier(struct_name)) => {
                token_iter.next();
                let name = match token_iter.next() {
                    Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
                    _ => panic!("identifier after type"),
                };
                expect_symbol(&mut token_iter, lexer::Symbol::Equals);
                let value = parse_struct::parse_data(&mut token_iter);
                ASTNode::Statement(Statement::VariableDeclaration {
                    var_type: VarType::Struct(struct_name.to_string()),
                    name: name.clone(),
                    value,
                })
            }
            _ => todo!(),
        };
        dbg!(&new_node);
        nodes.push(new_node);
    }
    ASTNode::Program(nodes)
}

fn parse_expression(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    let mut expr = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(id))) => Expression::Identifier(id.clone()),
        Some(Token::Literal(lit)) => parse_literal(lit),
        Some(Token::Symbol(lexer::Symbol::LeftBracket)) => {
            println!("its a fucking vec!!");
            parse_vec::parse_data(token_iter)
        }
        _ => panic!("literal or identifier"),
    };
    while let Some(token) = token_iter.peek() {
        match token {
            Token::Symbol(lexer::Symbol::Plus) => {
                token_iter.next();
                let right = match token_iter.next() {
                    Some(Token::Literal(lit)) => parse_literal(lit),
                    _ => panic!("literal after +"),
                };
                expr = Expression::BinaryOperation {
                    left: Box::new(expr),
                    op: BinaryOp::Add,
                    right: Box::new(right),
                };
            }
            _ => break,
        }
    }
    expr
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{self, Token};

    #[test]
    fn vec_type() {
        let input = lexer::lex("vec(int)");
        let output = VarType::Vec(Box::new(VarType::Int));
        let result = parse_types::parse(&mut input.iter().peekable());
        assert_eq!(output, result);
    }

    #[test]
    fn literal_int() {
        let input = lexer::Literal::Number("5".into());
        let output = Expression::Literal(Literal::Int(5));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_string() {
        let input = lexer::Literal::String("balls".into());
        let output = Expression::Literal(Literal::String("balls".into()));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_bool() {
        let input = lexer::Literal::Identifier("false".into());
        let output = Expression::Literal(Literal::Bool(false));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_identifier() {
        let input = lexer::Literal::Identifier("pepsi".into());
        let output = Expression::Identifier("pepsi".into());
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_float() {
        let input = lexer::Literal::Number("3.14".into());
        #[allow(clippy::approx_constant)]
        let output = Expression::Literal(Literal::Float(3.14));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }
    #[test]
    fn basic_identifier() {
        let input = vec![
            lexer::Token::Keyword(lexer::Keyword::Int),
            lexer::Token::Literal(lexer::Literal::Identifier("x".to_string())),
            lexer::Token::Symbol(lexer::Symbol::Equals),
            lexer::Token::Literal(lexer::Literal::Number("5".into())),
        ];
        let output = ASTNode::Statement(Statement::VariableDeclaration {
            var_type: VarType::Int,
            name: "x".into(),
            value: Expression::Literal(Literal::Int(5)),
        });
        let program = ASTNode::Program(vec![output]);
        let result = parse(input);
        assert_eq!(program, result);
        dbg!(result);
    }

    #[test]
    fn addition_expression() {
        let input = [
            Token::Literal(lexer::Literal::Number("5".to_string())),
            Token::Symbol(lexer::Symbol::Plus),
            Token::Literal(lexer::Literal::Number("2".to_string())),
        ];
        let mut token_iter = input.iter().peekable();
        let output = Expression::BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(5))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(2))),
        };
        let result = parse_expression(&mut token_iter);
        assert_eq!(output, result)
    }
}
