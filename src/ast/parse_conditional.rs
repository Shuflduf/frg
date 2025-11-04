use super::*;

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    let conditional_type = match token_iter.next() {
        Some(Token::Keyword(lexer::Keyword::If)) => {
            ConditionalType::If(parse_expression(token_iter))
        }
        Some(Token::Keyword(lexer::Keyword::Elif)) => {
            ConditionalType::Elif(parse_expression(token_iter))
        }
        Some(Token::Keyword(lexer::Keyword::Else)) => ConditionalType::Else,
        _ => panic!("wtf how"),
    };
    let body = parse_function::parse_body(token_iter);

    Statement::Conditional {
        conditional_type,
        body,
    }
}
