use std::env;

#[derive(Debug, PartialEq, Clone)]
enum Keyword {
    Struct,
    Void,
    Int,
    Str,
    Vec,
    Map,
    Set,
}

#[derive(Debug, PartialEq, Clone)]
enum Symbol {
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
}

#[derive(Debug, PartialEq, Clone)]
enum Literal {
    Int(i32),
    Float(f32),
    String(String),
    Identifier(String),
}

#[derive(Debug, Clone)]
enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Literal(Literal),
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: frg <code>.frg");
        return;
    }

    let file_path = &args[1];
}
