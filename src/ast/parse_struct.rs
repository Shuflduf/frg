use std::collections::HashMap;

use super::*;

pub fn parse_type(token_iter: &mut Peekable<Iter<Token>>) -> ASTNode {
    expect_token(token_iter, Token::Keyword(lexer::Keyword::Struct));
    let name = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
        _ => panic!("identifier after type"),
    };
    expect_symbol(token_iter, lexer::Symbol::LeftBrace);
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
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Comma)) {
            token_iter.next();
        }
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::RightBrace)) {
            token_iter.next();
            break;
        }
    }
    ASTNode::Statement(Statement::StructDeclaration {
        name: name.to_string(),
        fields,
    })
}

pub fn parse_data(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    expect_symbol(token_iter, lexer::Symbol::LeftBrace);
    let mut fields = HashMap::new();
    while let Some(token) = token_iter.next() {
        dbg!(token);
        let field_name = match token {
            Token::Literal(lexer::Literal::Identifier(id)) => id,
            _ => panic!("expected identifier"),
        };
        // if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Colon)) {
        //     token_iter.next();
        // }
        expect_symbol(token_iter, lexer::Symbol::Colon);
        let value = parse_expression(token_iter);
        fields.insert(field_name.to_string(), value);
        dbg!(token_iter.peek());
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Comma)) {
            token_iter.next();
        }
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::RightBrace)) {
            token_iter.next();
            break;
        }
    }
    Expression::CompositeLiteral(CompositeLiteral::Struct(fields))
}
