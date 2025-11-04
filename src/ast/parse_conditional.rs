use super::*;

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    expect_token(token_iter, Token::Keyword(lexer::Keyword::If));
    let value = parse_expression(token_iter);
    expect_symbol(token_iter, lexer::Symbol::LeftBrace);
    let mut body = vec![];

    while let Some(new_node) = parse_next(token_iter) {
        dbg!(&new_node);
        body.push(new_node);
        if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::RightBrace)) {
            token_iter.next();
            break;
        }
    }

    Statement::Conditional { value, body }
}
