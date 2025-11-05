use super::*;

pub fn parse_vec(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    let values = parse_list_of_values(token_iter, lexer::Symbol::RightBracket);
    Expression::CompositeLiteral(CompositeLiteral::Vec(values))
}

fn composite_type(mut token_iter: Peekable<Iter<Token>>) -> CompositeLiteral {
    while let Some(token) = token_iter.peek() {
        match token {
            Token::Symbol(lexer::Symbol::RightBrace) => return CompositeLiteral::Empty,
            Token::Symbol(lexer::Symbol::Comma) => {
                token_iter.next();
            }
            _ => {
                parse_expression(&mut token_iter);
                match token_iter.next() {
                    Some(Token::Symbol(lexer::Symbol::Colon)) => {
                        return CompositeLiteral::Map(vec![]);
                    }
                    _ => return CompositeLiteral::Set(vec![]),
                }
            }
        };
    }

    CompositeLiteral::Empty
}

pub fn parse_set_or_map(token_iter: &mut Peekable<Iter<Token>>) -> Expression {
    let composite = match composite_type(token_iter.clone()) {
        CompositeLiteral::Map(_) => parse_map(token_iter),
        CompositeLiteral::Set(_) => {
            CompositeLiteral::Set(parse_list_of_values(token_iter, lexer::Symbol::RightBrace))
        }
        CompositeLiteral::Empty => CompositeLiteral::Empty,
        _ => unreachable!(),
    };
    Expression::CompositeLiteral(composite)
}

fn parse_map(token_iter: &mut Peekable<Iter<Token>>) -> CompositeLiteral {
    let mut values = vec![];
    while let Some(token) = token_iter.peek() {
        match token {
            Token::Symbol(lexer::Symbol::Comma) => {
                token_iter.next();
            }
            Token::Symbol(lexer::Symbol::RightBrace) => {
                token_iter.next();
                break;
            }
            _ => {
                let key = parse_expression(token_iter);
                expect_symbol(token_iter, lexer::Symbol::Colon);
                let value = parse_expression(token_iter);
                values.push((key, value));
            }
        }
    }
    CompositeLiteral::Map(values)
}

fn parse_list_of_values(
    token_iter: &mut Peekable<Iter<Token>>,
    closing_symbol: lexer::Symbol,
) -> Vec<Expression> {
    let mut values = vec![];
    let closing_token = lexer::Token::Symbol(closing_symbol);
    while let Some(token) = token_iter.peek() {
        match token {
            Token::Symbol(lexer::Symbol::Comma) => {
                token_iter.next();
            }
            Token::Symbol(lexer::Symbol::RightParen) => {
                token_iter.next();
                break;
            }
            next => {
                if **next == closing_token {
                    token_iter.next();
                    break;
                } else {
                    values.push(parse_expression(token_iter))
                }
            }
        }
    }
    values
}
