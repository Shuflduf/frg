use super::*;
pub fn parse_params(
    mut token_iter: Peekable<Iter<Token>>,
) -> Option<(Vec<Parameter>, Peekable<Iter<Token>>)> {
    expect_symbol(&mut token_iter, lexer::Symbol::LeftParen);
    let mut params = vec![];

    while let Some(token) = token_iter.next() {
        dbg!(token);
        let new_param = match token {
            Token::Keyword(keyword) => {
                let param_type = parse_types::match_lexer_types(keyword);
                // if let token_iter.next()
                if let Some(Token::Literal(lexer::Literal::Identifier(param_name))) =
                    token_iter.next()
                {
                    dbg!(param_name);
                    Parameter {
                        name: param_name.to_string(),
                        param_type,
                    }
                } else {
                    return None;
                }
            }
            Token::Symbol(lexer::Symbol::RightParen) => break,
            _ => return None,
        };
        params.push(new_param);
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Comma)) {
            token_iter.next();
        }
    }
    Some((params, token_iter))
}
