use super::*;

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    let var_type = parse_types::parse(token_iter);
    let name = match token_iter.next() {
        Some(Token::Literal(lexer::Literal::Identifier(n))) => n,
        _ => panic!("identifier after type"),
    };
    expect_symbol(token_iter, lexer::Symbol::Equals);
    if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::LeftParen))
        && let Some((params, new_token_iter)) = parse_function::parse_params(token_iter.clone())
    {
        {
            dbg!(&params);
            *token_iter = new_token_iter;
            Statement::FunctionDeclaration {
                return_type: var_type,
                name: name.clone(),
                params,
                body: parse_function::parse_body(token_iter),
            }
        }
    } else {
        let value = parse_expression(token_iter);
        Statement::VariableDeclaration {
            var_type,
            name: name.clone(),
            value,
        }
    }
}
