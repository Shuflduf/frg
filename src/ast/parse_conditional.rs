use super::*;

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    expect_token(token_iter, Token::Keyword(lexer::Keyword::If));
    let value = parse_expression(token_iter);
    let body = parse_function::parse_body(token_iter);

    Statement::Conditional { value, body }
}
