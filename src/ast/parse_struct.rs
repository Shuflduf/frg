use std::collections::HashMap;

use super::*;

pub fn is_struct_definition(mut token_iter: Peekable<Iter<Token>>) -> bool {
    token_iter.next();
    if let Some(Token::Literal(lexer::Literal::Identifier(_struct_variable_name))) =
        token_iter.next()
    {
        return true;
    }
    false
}

pub fn parse_type(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
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
            param_type: parse_types::match_lexer_types(var_type),
        });
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Comma)) {
            token_iter.next();
        }
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::RightBrace)) {
            token_iter.next();
            break;
        }
    }
    Statement::StructDeclaration {
        name: name.to_string(),
        fields,
    }
}

pub fn parse_declaration(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    let Token::Literal(lexer::Literal::Identifier(struct_name)) = token_iter.next().unwrap() else {
        unreachable!()
    };
    let name = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
        _ => panic!("identifier after type"),
    };
    expect_symbol(token_iter, lexer::Symbol::Equals);
    let value = parse_struct::parse_data(token_iter);
    Statement::VariableDeclaration {
        var_type: VarType::Struct(struct_name.to_string()),
        name: name.clone(),
        value,
    }
}

fn parse_data(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    expect_symbol(token_iter, lexer::Symbol::LeftBrace);
    let mut fields = HashMap::new();
    while let Some(token) = token_iter.next() {
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
