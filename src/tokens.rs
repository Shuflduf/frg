use std::fmt::Display;

use chumsky::span::{SimpleSpan, Spanned};
use logos::Logos;
use strum_macros::Display;

use crate::errors::Diagnostic;

#[derive(Default, Clone, PartialEq, Debug)]
pub enum LexError {
    #[default]
    Other,
    InvalidEscape(String),
    InvalidNumber(String),
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Other => f.write_str("invalid token"),
            Self::InvalidEscape(escape) => write!(f, "invalid escape sequence `{escape}`"),
            Self::InvalidNumber(number) => write!(f, "invalid number `{number}`"),
        }
    }
}

#[derive(Logos, PartialEq, Clone, Display)]
#[logos(error = LexError)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'src> {
    #[token("=")]
    #[strum(to_string = "=")]
    Equals,
    #[token("(")]
    #[strum(to_string = "(")]
    LeftParen,
    #[token(")")]
    #[strum(to_string = ")")]
    RightParen,
    #[token("{")]
    #[strum(to_string = "{")]
    LeftBrace,
    #[token("}")]
    #[strum(to_string = "}}")]
    RightBrace,
    #[token("[")]
    #[strum(to_string = "[")]
    LeftBracket,
    #[token("]")]
    #[strum(to_string = "]")]
    RightBracket,
    #[token("+")]
    #[strum(to_string = "+")]
    Plus,
    #[token("-")]
    #[strum(to_string = "-")]
    Minus,
    #[token("*")]
    #[strum(to_string = "*")]
    Star,
    #[token("/")]
    #[strum(to_string = "/")]
    Slash,
    #[token("!")]
    #[strum(to_string = "!")]
    Exclamation,
    #[token("+=")]
    #[strum(to_string = "+=")]
    PlusEquals,
    #[token("-=")]
    #[strum(to_string = "-=")]
    MinusEquals,
    #[token("*=")]
    #[strum(to_string = "*=")]
    StarEquals,
    #[token("/=")]
    #[strum(to_string = "/=")]
    SlashEquals,
    #[token("==")]
    #[strum(to_string = "==")]
    Equivalent,
    #[token("!=")]
    #[strum(to_string = "!=")]
    NotEquivalent,

    #[token("int")]
    #[strum(to_string = "int")]
    IntType,
    #[token("float")]
    #[strum(to_string = "float")]
    FloatType,
    #[token("bool")]
    #[strum(to_string = "bool")]
    BoolType,
    #[token("list")]
    #[token("arr")]
    #[token("array")]
    #[token("vec")]
    #[strum(to_string = "list")]
    ListType,
    #[token("map")]
    #[token("dict")]
    #[strum(to_string = "map")]
    MapType,
    #[token("type")]
    #[strum(to_string = "type")]
    TypeType,
    #[token("struct")]
    #[strum(to_string = "struct")]
    StructDecl,
    #[token("enum")]
    #[strum(to_string = "enum")]
    EnumDecl,

    #[token("if")]
    #[strum(to_string = "if")]
    If,
    #[token("else")]
    #[strum(to_string = "else")]
    Else,

    #[token("true")]
    #[strum(to_string = "true")]
    BoolTrue,
    #[token("false")]
    #[strum(to_string = "false")]
    BoolFalse,

    #[regex("[a-z_][a-zA-Z0-9_]*", |lex| lex.slice())]
    Identifier(&'src str),

    #[regex("[0-9]+", |lex| lex.slice().parse::<i64>().map_err(|_| LexError::InvalidNumber(lex.slice().to_owned())))]
    IntLiteral(i64),

    #[regex(r"([0-9]+\.[0-9]*|\.[0-9]+)([eE][+-]?[0-9]+)?", |lex| {
        lex.slice().parse::<f64>().map_err(|_| LexError::InvalidNumber(lex.slice().to_owned()))
    })]
    FloatLiteral(f64),

    #[regex(r#""([^"\\]|\\.)*""#, |lex| {let s = lex.slice(); unescape(&s[1..s.len()-1]) })]
    StringLiteral(String),
}

pub fn lex<'src>(src: &'src str) -> Result<Vec<Spanned<Token<'src>>>, Diagnostic> {
    let mut l = Token::lexer(src);
    let mut tokens = vec![];
    while let Some(result) = l.next() {
        let span = SimpleSpan::from(l.span());
        match result {
            Ok(token) => tokens.push(Spanned { inner: token, span }),
            Err(_) => {
                let text = &src[span.into_range()];
                return Err(
                    Diagnostic::new(format!("invalid token `{text}`")).with_label("here", span)
                );
            }
        }
    }
    Ok(tokens)
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
