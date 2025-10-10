use super::*;
pub fn parse_data(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    // dbg!(token_iter.next());
    let mut values = vec![];
    while let Some(token) = token_iter.peek() {
        dbg!(token);
        match token {
            Token::Symbol(lexer::Symbol::RightBracket) => {
                token_iter.next();
                break;
            }
            Token::Symbol(lexer::Symbol::Comma) => {
                token_iter.next();
            }
            _ => values.push(parse_expression(token_iter)),
        }
    }

    Expression::CompositeLiteral(CompositeLiteral::Vec(values))
}
