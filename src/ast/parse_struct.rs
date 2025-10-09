use super::*;

pub fn parse(token_iter: &mut Iter<Token>) -> ASTNode {
    let name = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
        _ => panic!("identifier after type"),
    };
    match token_iter.next() {
        Some(Token::Symbol(lexer::Symbol::LeftBrace)) => {}
        _ => panic!("{{ after identifier"),
    }
    let mut fields: Vec<Parameter> = vec![];
    while let Some(token) = token_iter.next() {
        let var_type = match token {
            Token::Keyword(n) => n,
            _ => panic!("type expected"),
        };
        let field_name = match token_iter.next() {
            Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
            _ => panic!("identifier after type"),
        };
        fields.push(Parameter {
            name: field_name.to_string(),
            param_type: match_lexer_types(var_type),
        });
        match token_iter.next() {
            Some(Token::Symbol(lexer::Symbol::Comma)) => {}
            Some(Token::Symbol(lexer::Symbol::RightBrace)) => break,
            _ => panic!("expected , or }}"),
        }
    }
    ASTNode::Statement(Statement::StructDeclaration {
        name: name.to_string(),
        fields,
    })
}
