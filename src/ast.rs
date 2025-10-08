use crate::{
    ast_nodes::*,
    lexer::{self, Token},
};

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    let mut nodes = vec![];
    let mut token_iter = tokens.iter().peekable();
    while let Some(token) = token_iter.next() {
        match token {
            // TODO:
            Token::Keyword(lexer::Keyword::Int) => {
                let name = match token_iter.next() {
                    Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
                    _ => panic!("identifier after type"),
                };
                match token_iter.next() {
                    Some(Token::Symbol(lexer::Symbol::Equals)) => {}
                    _ => panic!("= after identifier"),
                }
                let value = match token_iter.next() {
                    Some(Token::Literal(lexer::Literal::Number(n))) => {
                        Expression::Literal(Literal::Int(n.parse().unwrap()))
                    }
                    _ => panic!("value after ="),
                };
                nodes.push(ASTNode::Statement(Statement::VariableDeclaration {
                    var_type: VarType::Int,
                    name: name.clone(),
                    value,
                }));
            }
            _ => todo!(),
        }
        dbg!(&token);
    }
    return ASTNode::Program(nodes);
}

// shouldnt be pub but idc
pub fn parse_literal(literal: &lexer::Literal) -> Expression {
    match literal {
        lexer::Literal::Number(new_num) => {
            if is_num_float(&new_num) {
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
    use crate::{
        ast::{parse, parse_literal},
        ast_nodes::*,
        lexer,
    };

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
}
