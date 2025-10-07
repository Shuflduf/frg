use crate::{
    ast_nodes::{self, *},
    lexer::{self, Token},
};

pub fn parse(tokens: Vec<Token>) -> ASTNode {
    return ASTNode::Program(vec![]);
}

fn parse_literal(literal: &lexer::Literal) -> Expression {
    match literal {
        // TODO: floats
        lexer::Literal::Number(new_num) => {
            Expression::Literal(Literal::Int(new_num.parse().unwrap()))
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
    use crate::{ast::parse, ast_nodes, lexer};

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
