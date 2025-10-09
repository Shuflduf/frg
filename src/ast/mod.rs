use std::slice::Iter;

use crate::{
    ast::ast_nodes::*,
    lexer::{self, Token},
};

mod ast_nodes;
mod parse_struct;

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = vec![];
    let mut token_iter = tokens.iter();
    while let Some(token) = token_iter.next() {
        let new_node = match token {
            Token::Keyword(lexer::Keyword::Struct) => parse_struct::parse(&mut token_iter),
            Token::Keyword(var_type) => {
                // i still dont understand rust why do i need to clone
                let var_type = parse_type(Some(&Token::Keyword(var_type.clone())), &mut token_iter);
                let name = match token_iter.next() {
                    Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
                    _ => panic!("identifier after type"),
                };
                match token_iter.next() {
                    Some(Token::Symbol(lexer::Symbol::Equals)) => {}
                    _ => panic!("= after identifier"),
                }
                let value = parse_expression(&mut token_iter);
                ASTNode::Statement(Statement::VariableDeclaration {
                    var_type,
                    name: name.clone(),
                    value,
                })
            }
            _ => todo!(),
        };
        nodes.push(new_node);
    }
    ASTNode::Program(nodes)
}

pub fn match_lexer_types(lexer_type: &lexer::Keyword) -> VarType {
    match lexer_type {
        lexer::Keyword::Void => VarType::Void,
        lexer::Keyword::Int => VarType::Int,
        lexer::Keyword::Float => VarType::Float,
        lexer::Keyword::Bool => VarType::Bool,
        lexer::Keyword::Str => VarType::Str,
        _ => todo!(),
    }
}

pub fn parse_type(last_token: Option<&Token>, token_iter: &mut Iter<Token>) -> VarType {
    match last_token.or_else(|| token_iter.next()) {
        Some(Token::Keyword(lexer::Keyword::Vec)) => {
            match token_iter.next() {
                Some(Token::Symbol(lexer::Symbol::LeftParen)) => {}
                _ => panic!("( after identifier"),
            }
            let var_type = parse_type(None, token_iter);
            match token_iter.next() {
                Some(Token::Symbol(lexer::Symbol::RightParen)) => {}
                _ => panic!(") after identifier"),
            }
            VarType::Vec(Box::new(var_type))
        }
        Some(Token::Keyword(var_type)) => match_lexer_types(&var_type),
        _ => todo!(),
    }
}

fn parse_expression(token_iter: &mut Iter<Token>) -> Expression {
    let mut expr = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(id))) => Expression::Identifier(id.clone()),
        Some(Token::Literal(lit)) => parse_literal(lit),
        _ => panic!("literal or identifier"),
    };
    while let Some(token) = token_iter.next() {
        match token {
            Token::Symbol(lexer::Symbol::Plus) => {
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
            if is_num_float(new_num) {
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

fn is_num_float(num: &str) -> bool {
    num.contains(".")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::{self, Token};

    #[test]
    fn vec_type() {
        let input = lexer::lex("vec(int)");
        let output = VarType::Vec(Box::new(VarType::Int));
        let result = parse_type(None, &mut input.iter());
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
        let mut token_iter = input.iter();
        let output = Expression::BinaryOperation {
            left: Box::new(Expression::Literal(Literal::Int(5))),
            op: BinaryOp::Add,
            right: Box::new(Expression::Literal(Literal::Int(2))),
        };
        let result = parse_expression(&mut token_iter);
        assert_eq!(output, result)
    }
}
