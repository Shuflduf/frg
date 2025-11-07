use super::*;

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> Statement {
    token_iter.next();
    let loop_over = parse_expression(token_iter);
    commas(token_iter);
    if let Some(Token::Literal(lexer::Literal::Identifier(index_var))) = token_iter.next() {
        Statement::Loop {
            loop_over,
            index_var: index_var.to_string(),
            body: parse_function::parse_body(token_iter),
        }
    } else {
        panic!("expected identifier after loop target")
    }
    // while let Some(token) = token_iter.next() {
    //     match token {
    //         Token::Symbol(lexer::Symbol::Comma) => continue,
    //         Token::Literal(lexer::Literal::Identifier(index_var)) => {
    //             let body = parse_function::parse_body(token_iter);
    //         }
    //     }
    // }
}
