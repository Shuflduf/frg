use super::*;

pub fn match_lexer_types(lexer_type: &lexer::Keyword) -> VarType {
    match lexer_type {
        lexer::Keyword::Void => VarType::Void,
        lexer::Keyword::Int => VarType::Int,
        lexer::Keyword::Float => VarType::Float,
        lexer::Keyword::Bool => VarType::Bool,
        lexer::Keyword::Str => VarType::Str,
        _ => todo!(),
    }
}

pub fn parse(token_iter: &mut Peekable<Iter<Token>>) -> VarType {
    match token_iter.next() {
        Some(Token::Keyword(lexer::Keyword::Vec)) => {
            expect_symbol(token_iter, lexer::Symbol::LeftParen);
            let var_type = parse(token_iter);
            expect_symbol(token_iter, lexer::Symbol::RightParen);
            VarType::Vec(Box::new(var_type))
        }
        // literally copy pasted
        Some(Token::Keyword(lexer::Keyword::Set)) => {
            expect_symbol(token_iter, lexer::Symbol::LeftParen);
            let var_type = parse(token_iter);
            expect_symbol(token_iter, lexer::Symbol::RightParen);
            VarType::Set(Box::new(var_type))
        }
        Some(Token::Keyword(lexer::Keyword::Map)) => {
            expect_symbol(token_iter, lexer::Symbol::LeftParen);
            let key_type = parse(token_iter);

            if token_iter.peek() == Some(&&Token::Symbol(lexer::Symbol::Comma)) {
                token_iter.next();
            }
            let value_type = parse(token_iter);
            expect_symbol(token_iter, lexer::Symbol::RightParen);
            // VarType::Vec(Box::new(var_type))
            VarType::Map(Box::new(key_type), Box::new(value_type))
        }
        Some(Token::Keyword(var_type)) => match_lexer_types(var_type),
        _ => todo!(),
    }
}
