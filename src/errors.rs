use std::{collections::HashSet, fmt::Display};

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use chumsky::{
    error::{Rich, RichPattern},
    span::{SimpleSpan, Spanned},
};

use crate::tokens::Token;

pub struct Diagnostic {
    message: String,
    labels: Vec<Spanned<String>>,
}

// pub struct DiagnosticLabel {
//     pub span: SimpleSpan,
//     pub message: String,
// }

pub enum OpError {
    InvalidBinaryOp {
        op: &'static str,
        left: &'static str,
        right: &'static str,
    },
    InvalidUnaryOp {
        op: &'static str,
        expr: &'static str,
    },
    DivisionByZero,
}

pub struct DelimiterCheck {
    pub diagnostics: Vec<Diagnostic>,
    pub can_parse: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Delimiter {
    Paren,
    Bracket,
    Brace,
}

impl Diagnostic {
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            labels: vec![],
        }
    }

    pub fn basic(message: impl Into<String>, span: SimpleSpan) -> Self {
        let m = message.into();
        Self::new(m.clone()).with_label(m, span)
    }

    pub fn with_label(mut self, message: impl Into<String>, span: SimpleSpan) -> Self {
        self.labels.push(Spanned {
            inner: message.into(),
            span,
        });
        self
    }

    pub fn report(&self, source: &str, filename: &str) {
        let mut colours = ColorGenerator::new();
        let mut labels: Vec<&Spanned<String>> = self.labels.iter().collect();
        labels.sort_by_key(|label| label.span.start);

        let primary = labels
            .first()
            .map(|label| label.span)
            .unwrap_or_else(|| SimpleSpan::from(0..0));

        let labels: Vec<Label<(&str, std::ops::Range<usize>)>> = labels
            .into_iter()
            .map(|label| {
                Label::new((filename, label.span.into_range()))
                    .with_message(&label.inner)
                    .with_color(colours.next())
            })
            .collect();

        Report::build(ReportKind::Error, (filename, primary.start..primary.end))
            .with_message(&self.message)
            .with_labels(labels)
            .finish()
            .eprint((filename, Source::from(source)))
            .unwrap();
    }

    pub fn primary_span(&self) -> Option<SimpleSpan> {
        self.labels.first().map(|label| label.span)
    }

    pub fn from_parse_error(
        error: &Rich<'_, Token<'_>>,
        tokens: &[Spanned<Token<'_>>],
        source: &str,
    ) -> Self {
        let span = display_span(*error.span(), tokens);
        let context = error
            .contexts()
            .next()
            .map(|(pattern, span)| match pattern {
                RichPattern::Token(token) => {
                    source_spelling(source, *span).unwrap_or_else(|| token.to_string())
                }
                _ => pattern.to_string(),
            })
            .unwrap_or_else(|| "input".into());

        let found = error
            .found()
            .map(|token| {
                source_spelling(source, *error.span()).unwrap_or_else(|| token.to_string())
            })
            .unwrap_or_else(|| "end of input".into());
        let mut seen = HashSet::new();
        let expected: Vec<String> = error
            .expected()
            .filter_map(|pattern| {
                let text = match pattern {
                    RichPattern::Token(token) => (**token).to_string(),
                    RichPattern::Label(label) => label.to_string(),
                    RichPattern::Identifier(ident) => ident.to_string(),
                    RichPattern::Any => "any".into(),
                    RichPattern::SomethingElse => "something else".into(),
                    RichPattern::EndOfInput => "end of input".into(),
                    _ => pattern.to_string(),
                };
                seen.insert(text.clone()).then_some(text)
            })
            .collect();
        let expected = match expected.len() {
            0 => "expected something else".into(),
            1 => format!("expected {}", expected[0]),
            _ => format!("expected one of {}", expected.join(", ")),
        };

        let mut diagnostic = Diagnostic::new(format!(
            "unexpected {found} while parsing {context}, {expected}"
        ))
        .with_label(format!("unexpected {found}"), span);

        let in_var_declaration = error.contexts().any(|(pattern, _)| {
            matches!(pattern, RichPattern::Label(label) if label.as_ref() == "variable declaration")
        });
        if in_var_declaration
            && let Some(equals) = previous_tokens(*error.span(), tokens)
            && equals.inner == Token::Equals
            && equals.span != span
        {
            diagnostic = diagnostic.with_label(
                format!("expected an expression after this `=`"),
                equals.span,
            );
        }

        diagnostic
    }
}

impl Delimiter {
    fn open(&self) -> &'static str {
        match self {
            Self::Paren => "(",
            Self::Bracket => "[",
            Self::Brace => "{",
        }
    }
    fn close(&self) -> &'static str {
        match self {
            Self::Paren => ")",
            Self::Bracket => "]",
            Self::Brace => "}",
        }
    }

    fn context(&self) -> &'static str {
        match self {
            Self::Paren | Self::Bracket => "expression",
            Self::Brace => "block",
        }
    }
}

pub fn check_delimiters(tokens: &[Spanned<Token<'_>>]) -> DelimiterCheck {
    let mut stack = Vec::new();
    let mut diagnostics = Vec::new();
    let mut can_parse = true;

    for token in tokens {
        let Some((kind, is_open)) = delimiter_of(&token.inner) else {
            continue;
        };

        if is_open {
            stack.push((kind, token.span));
            continue;
        }

        match stack.pop() {
            None => {
                let close = kind.close();
                let span = token.span;
                diagnostics.push(
                    Diagnostic::new(format!("unexpected closing delimiter `{close}`")).with_label(
                        format!("this `{close}` has no matching opening delimiter"),
                        span,
                    ),
                );
            }
            Some((open_kind, _)) if open_kind == kind => {}
            Some((open_kind, open_span)) => {
                let open = open_kind.open();
                let close = kind.close();
                let expected = open_kind.close();
                let context = open_kind.context();
                diagnostics.push(
                    Diagnostic::new(format!(
                        "unclosed delimiter `{open}` while parsing {context}, expected `{expected}`"
                    ))
                    .with_label(format!("delimiter `{open}` is never closed"), open_span)
                    .with_label(format!("must be closed before this `{close}`"), token.span),
                );
                can_parse = false;
                return DelimiterCheck {
                    diagnostics,
                    can_parse,
                };
                // diagnostics.push(value);
            }
        }
    }
    if !stack.is_empty() {
        can_parse = false;
        diagnostics.extend(
            stack
                .into_iter()
                .rev()
                .map(|(kind, span)| unclosed_at_eof(kind, span)),
        );
    }
    DelimiterCheck {
        diagnostics,
        can_parse,
    }
}

fn unclosed_at_eof(kind: Delimiter, span: SimpleSpan) -> Diagnostic {
    let open = kind.open();
    let expected = kind.close();
    let context = kind.context();
    Diagnostic::new(format!(
        "unclosed delimiter `{open}` while parsing {context}, expected `{expected}`"
    ))
    .with_label(format!("delimiter `{open}` is never closed"), span)
}

fn delimiter_of(token: &Token<'_>) -> Option<(Delimiter, bool)> {
    match token {
        Token::LeftParen => Some((Delimiter::Paren, true)),
        Token::RightParen => Some((Delimiter::Paren, false)),
        Token::LeftBracket => Some((Delimiter::Bracket, true)),
        Token::RightBracket => Some((Delimiter::Bracket, false)),
        Token::LeftBrace => Some((Delimiter::Brace, true)),
        Token::RightBrace => Some((Delimiter::Brace, false)),
        _ => None,
    }
}

fn previous_tokens<'tokens, 'src>(
    span: SimpleSpan,
    tokens: &'tokens [Spanned<Token<'src>>],
) -> Option<&'tokens Spanned<Token<'src>>> {
    tokens
        .iter()
        .rev()
        .find(|token| token.span.start <= span.start)
}

fn display_span(span: SimpleSpan, tokens: &[Spanned<Token<'_>>]) -> SimpleSpan {
    if span.start < span.end {
        return span;
    }

    previous_tokens(span, tokens)
        .map(|token| token.span)
        .unwrap_or(span)
}

fn source_spelling(source: &str, span: SimpleSpan) -> Option<String> {
    let text = source.get(span.into_range())?;
    (!text.is_empty()).then(|| text.to_owned())
}

// impl Display for Diagnostic {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(&self.message)
//     }
// }

impl Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBinaryOp { op, left, right } => {
                write!(f, "cannot apply `{op}` to `{left}` and `{right}`")
            }
            Self::InvalidUnaryOp { op, expr } => {
                write!(f, "cannot apply `{op}` to `{expr}`")
            }
            Self::DivisionByZero => f.write_str("division by zero"),
        }
    }
}
