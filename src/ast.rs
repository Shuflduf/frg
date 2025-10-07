use crate::{
    ast_nodes::{self, *},
    lexer::{self, Token},
};

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    return ASTNode::Program(vec![]);
}

// shouldnt be pub but idc
pub fn parse_literal(literal: &lexer::Literal) -> Expression {
    match literal {
        // TODO: floats
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
        ast_nodes, lexer,
    };

    #[test]
    fn literal_int() {
        let input = lexer::Literal::Number("5".into());
        let output = ast_nodes::Expression::Literal(ast_nodes::Literal::Int(5));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_string() {
        let input = lexer::Literal::String("balls".into());
        let output = ast_nodes::Expression::Literal(ast_nodes::Literal::String("balls".into()));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_bool() {
        let input = lexer::Literal::Identifier("false".into());
        let output = ast_nodes::Expression::Literal(ast_nodes::Literal::Bool(false));
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_identifier() {
        let input = lexer::Literal::Identifier("pepsi".into());
        let output = ast_nodes::Expression::Identifier("pepsi".into());
        let result = parse_literal(&input);
        assert_eq!(output, result);
    }

    #[test]
    fn literal_float() {
        let input = lexer::Literal::Number("3.14".into());
        let output = ast_nodes::Expression::Literal(ast_nodes::Literal::Float(3.14));
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
        let output = ast_nodes::ASTNode::Statement(ast_nodes::Statement::VariableDeclaration {
            var_type: ast_nodes::VarType::Int,
            name: "x".into(),
            value: ast_nodes::Expression::Literal(ast_nodes::Literal::Int(5)),
        });
        let result = parse(input);
        assert_eq!(output, result);
        dbg!(result);
    }
}
