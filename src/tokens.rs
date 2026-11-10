use std::ops::Range;

use logos::Logos;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum LexError {
    #[default]
    Other,
    InvalidEscape(String),
}

#[derive(Debug)]
pub struct TokenData<'src> {
    token: Token<'src>,
    span: Range<usize>,
}

#[derive(Logos, Debug)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'src> {
    #[token("int")]
    IntType,

    #[token("=")]
    Equals,

    #[regex("[a-z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'src str),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().unwrap())]
    IntLiteral(i64),

    #[regex(r"([0-9]+\.[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?", |lex| lex.slice().parse::<f64>().unwrap())]
    FloatLiteral(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {let s = lex.slice(); unescape(&s[1..s.len()-1]) })]
    StringLiteral(String),
}

pub fn lex(src: &str) -> Vec<TokenData> {
    let mut l = Token::lexer(src);
    let mut tokens = vec![];
    while let Some(Ok(next)) = l.next() {
        tokens.push(TokenData {
            token: next,
            span: l.span(),
        });
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
