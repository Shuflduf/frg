#[derive(Debug, PartialEq, Clone)]
pub enum Keyword {
    Struct,
    Void,
    Int,
    Float,
    Str,
    Vec,
    Map,
    Set,
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
    Comma,
    Ampersand,
    Star,
    Colon,
    Period,
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

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Lexer {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Token> {
        if !self.input.ends_with(&[' ']) {
            self.input.push(' ');
        }

        let mut token_list: Vec<Token> = vec![];
        let mut string_open = false;
        let mut current_token = String::new();
        let mut char_iter = self.input.iter().peekable();
        while let Some(c) = &mut char_iter.next() {
            let c = **c;
            dbg!(c);
            // if string_open {
            //     if c == '"' {
            //         current_token.push(c);
            //     } else {
            //         string_open = false
            //     }
            // } else if c == '"' {
            //     string_open = true
            // }
            let new_symbol_token = match c {
                '{' => Some(Token::Symbol(Symbol::LeftBrace)),
                '}' => Some(Token::Symbol(Symbol::RightBrace)),
                '(' => Some(Token::Symbol(Symbol::LeftParen)),
                ')' => Some(Token::Symbol(Symbol::RightParen)),
                '[' => Some(Token::Symbol(Symbol::LeftBracket)),
                ']' => Some(Token::Symbol(Symbol::RightBracket)),
                '=' => Some(Token::Symbol(Symbol::Equals)),
                ',' => Some(Token::Symbol(Symbol::Comma)),
                '&' => Some(Token::Symbol(Symbol::Ampersand)),
                '*' => Some(Token::Symbol(Symbol::Star)),
                ':' => Some(Token::Symbol(Symbol::Colon)),
                '.' => Some(Token::Symbol(Symbol::Period)),
                _ => None,
            };
            let add_current_token = new_symbol_token.is_some() || c == ' ';
            if add_current_token && !current_token.is_empty() {
                let prev_token = match current_token.as_str() {
                    "struct" => Token::Keyword(Keyword::Struct),
                    "void" => Token::Keyword(Keyword::Void),
                    "int" => Token::Keyword(Keyword::Int),
                    "float" => Token::Keyword(Keyword::Float),
                    "str" => Token::Keyword(Keyword::Str),
                    "vec" => Token::Keyword(Keyword::Vec),
                    "map" => Token::Keyword(Keyword::Map),
                    "set" => Token::Keyword(Keyword::Set),
                    _ => Token::Literal(Literal::Identifier(current_token)),
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
                dbg!(&current_token);
            }
        }
        token_list
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keyword_test() {
        let input = ": ()&frog int *)";
        let output = vec![
            Token::Symbol(Symbol::Colon),
            Token::Symbol(Symbol::LeftParen),
        ];
        let result = Lexer::new(input).parse();
        assert_eq!(output, result);
    }

    #[ignore]
    #[test]
    fn basic_identifier() {
        let input = "int x = 5";
        let output = vec![
            Token::Keyword(Keyword::Int),
            Token::Literal(Literal::Identifier("x".into())),
            Token::Symbol(Symbol::Equals),
            Token::Literal(Literal::Number("5".into())),
        ];
        let result = Lexer::new(input).parse();
        assert_eq!(output, result);
    }
}
