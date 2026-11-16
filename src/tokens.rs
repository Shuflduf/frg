use chumsky::span::{SimpleSpan, Spanned};
use logos::Logos;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum LexError {
    #[default]
    Other,
    InvalidEscape(String),
}
#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'src> {
    #[token("=")]
    Equals,
    #[token("(")]
    LeftParen,
    #[token(")")]
    RightParen,
    #[token("{")]
    LeftBrace,
    #[token("}")]
    RightBrace,
    #[token("[")]
    LeftBracket,
    #[token("]")]
    RightBracket,
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("/")]
    Slash,
    #[token("!")]
    Exclamation,
    #[token("+=")]
    PlusEquals,
    #[token("-=")]
    MinusEquals,
    #[token("*=")]
    StarEquals,
    #[token("/=")]
    SlashEquals,
    #[token("==")]
    Equivalent,
    #[token("!=")]
    NotEquivalent,

    #[token("int")]
    IntType,
    #[token("float")]
    FloatType,
    #[token("list")]
    #[token("arr")]
    #[token("array")]
    #[token("vec")]
    ListType,
    #[token("map")]
    #[token("dict")]
    MapType,
    #[token("type")]
    TypeType,
    #[token("struct")]
    StructDecl,
    #[token("enum")]
    EnumDecl,

    #[token("if")]
    If,
    #[token("else")]
    Else,

    #[regex("[a-z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'src str),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLiteral(i64),

    #[regex(r"([0-9]+\.[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    FloatLiteral(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {let s = lex.slice(); unescape(&s[1..s.len()-1]) })]
    StringLiteral(String),
}

pub fn lex<'src>(src: &'src str) -> Vec<Spanned<Token<'src>>> {
    let mut l = Token::lexer(src);
    let mut tokens = vec![];
    while let Some(result) = l.next() {
        match result {
            Ok(token) => tokens.push(Spanned {
                inner: token,
                span: SimpleSpan::from(l.span()),
            }),
            Err(_) => todo!(),
        }
    }
    tokens
}

fn unescape(raw: &str) -> Result<String, LexError> {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars();

    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('n') => out.push('\n'),
            Some('t') => out.push('\t'),
            Some('r') => out.push('\r'),
            Some('0') => out.push('\0'),
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some('u') => {
                if chars.next() != Some('{') {
                    return Err(LexError::InvalidEscape("\\u".into()));
                }
                let hex: String = chars.by_ref().take_while(|&c| c != '}').collect();
                let ch = u32::from_str_radix(&hex, 16)
                    .ok()
                    .and_then(char::from_u32)
                    .ok_or_else(|| LexError::InvalidEscape(format!("\\{hex}")))?;
                out.push(ch);
            }
            Some(other) => return Err(LexError::InvalidEscape(format!("\\{other}"))),
            None => return Err(LexError::InvalidEscape("\\".into())),
        }
    }
    Ok(out)
}
