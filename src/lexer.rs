#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Struct,
    Void,
    Int,
    Float,
    Bool,
    Str,
    Vec,
    Map,
    Set,
    Return,
    If,
    Elif,
    Else,
    Loop,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Symbol {
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Equals,
    DoubleEquals,
    Comma,
    Ampersand,
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    FSlash,
    FSlashEquals,
    Star,
    StarEquals,
    Colon,
    Period,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Number(String),
    String(String),
    Identifier(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut input = input.to_string();
    if !input.ends_with([' ']) {
        input.push(' ');
    }

    let mut token_list: Vec<Token> = vec![];
    let mut string_open = false;
    let mut current_token = String::new();
    let mut char_iter = input.chars().peekable();
    while let Some(c) = &mut char_iter.next() {
        let c = *c;
        if string_open {
            if c != '"' {
                current_token.push(c);
                continue;
            } else {
                token_list.push(Token::Literal(Literal::String(current_token)));
                current_token = String::new();
                string_open = false;
                continue;
            }
        } else if c == '"' {
            string_open = true;
            continue;
        }
        let new_symbol_token = match c {
            '{' => Some(Token::Symbol(Symbol::LeftBrace)),
            '}' => Some(Token::Symbol(Symbol::RightBrace)),
            '(' => Some(Token::Symbol(Symbol::LeftParen)),
            ')' => Some(Token::Symbol(Symbol::RightParen)),
            '[' => Some(Token::Symbol(Symbol::LeftBracket)),
            ']' => Some(Token::Symbol(Symbol::RightBracket)),
            ',' => Some(Token::Symbol(Symbol::Comma)),
            '&' => Some(Token::Symbol(Symbol::Ampersand)),
            '*' => Some(Token::Symbol(Symbol::Star)),
            '+' => Some(Token::Symbol(Symbol::Plus)),
            '-' => Some(Token::Symbol(Symbol::Minus)),
            '/' => Some(Token::Symbol(Symbol::FSlash)),
            ':' => Some(Token::Symbol(Symbol::Colon)),
            '<' => Some(Token::Symbol(Symbol::LessThan)),
            '>' => Some(Token::Symbol(Symbol::GreaterThan)),
            '=' => match token_list.last() {
                Some(Token::Symbol(Symbol::Equals)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::DoubleEquals))
                }
                Some(Token::Symbol(Symbol::LessThan)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::LessThanOrEqual))
                }
                Some(Token::Symbol(Symbol::GreaterThan)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::GreaterThanOrEqual))
                }
                Some(Token::Symbol(Symbol::Plus)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::PlusEquals))
                }
                Some(Token::Symbol(Symbol::Minus)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::MinusEquals))
                }
                Some(Token::Symbol(Symbol::FSlash)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::FSlashEquals))
                }
                Some(Token::Symbol(Symbol::Star)) => {
                    token_list.pop();
                    Some(Token::Symbol(Symbol::StarEquals))
                }
                _ => Some(Token::Symbol(Symbol::Equals)),
            },
            '.' => {
                if current_token.chars().next().unwrap().is_ascii_digit() {
                    current_token.push(c);
                    continue;
                } else {
                    Some(Token::Symbol(Symbol::Period))
                }
            } // this is weird with floats
            _ => None,
        };
        let add_current_token = new_symbol_token.is_some() || c == ' ' || c == '\n';
        if add_current_token && !current_token.is_empty() {
            let prev_token = match current_token.as_str() {
                "struct" => Token::Keyword(Keyword::Struct),
                "void" => Token::Keyword(Keyword::Void),
                "int" => Token::Keyword(Keyword::Int),
                "float" => Token::Keyword(Keyword::Float),
                "bool" => Token::Keyword(Keyword::Bool),
                "str" => Token::Keyword(Keyword::Str),
                "vec" => Token::Keyword(Keyword::Vec),
                "map" => Token::Keyword(Keyword::Map),
                "set" => Token::Keyword(Keyword::Set),
                "return" => Token::Keyword(Keyword::Return),
                "if" => Token::Keyword(Keyword::If),
                "else" => Token::Keyword(Keyword::Else),
                "elif" => Token::Keyword(Keyword::Elif),
                "loop" => Token::Keyword(Keyword::Loop),
                _ => Token::Literal(if current_token.chars().next().unwrap().is_ascii_digit() {
                    Literal::Number(current_token)
                } else {
                    Literal::Identifier(current_token)
                }),
            };
            token_list.push(prev_token);
            current_token = String::new();
        }
        if let Some(symbol_token) = new_symbol_token {
            token_list.push(symbol_token);
        } else {
            if c.is_whitespace() {
                continue;
            }
            current_token.push(c);
        }
    }
    token_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_test() {
        let input = "int target_frog = &frogs[0]";
        let output = vec![
            Token::Keyword(Keyword::Int),
            Token::Literal(Literal::Identifier("target_frog".into())),
            Token::Symbol(Symbol::Equals),
            Token::Symbol(Symbol::Ampersand),
            Token::Literal(Literal::Identifier("frogs".into())),
            Token::Symbol(Symbol::LeftBracket),
            Token::Literal(Literal::Number("0".into())),
            Token::Symbol(Symbol::RightBracket),
        ];
        let result = lex(input);
        assert_eq!(output, result);
        println!("{:?}", &result);
    }

    #[test]
    fn basic_identifier() {
        let input = "int x = 5";
        let output = vec![
            Token::Keyword(Keyword::Int),
            Token::Literal(Literal::Identifier("x".into())),
            Token::Symbol(Symbol::Equals),
            Token::Literal(Literal::Number("5".into())),
        ];
        let result = lex(input);
        assert_eq!(output, result);
        println!("{:?}", &result);
    }
}
