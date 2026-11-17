use std::fmt::Display;

use ariadne::{ColorGenerator, Label, Report, ReportKind, Source};
use chumsky::span::SimpleSpan;

pub struct RuntimeError {
    message: String,
    labels: Vec<RuntimeLabel>,
}

pub struct RuntimeLabel {
    pub span: SimpleSpan,
    pub message: String,
}

impl RuntimeError {
    pub fn new(message: impl Into<String>, span: SimpleSpan) -> Self {
        let message = message.into();
        Self {
            labels: vec![RuntimeLabel {
                span,
                message: message.clone(),
            }],
            message,
        }
    }

    pub fn with_labels(message: impl Into<String>, labels: Vec<RuntimeLabel>) -> Self {
        Self {
            message: message.into(),
            labels,
        }
    }

    pub fn report(&self, source: &str, filename: &str) {
        let mut colours = ColorGenerator::new();
        let labels: Vec<Label<(&str, std::ops::Range<usize>)>> = self
            .labels
            .iter()
            .map(|label| {
                Label::new((filename, label.span.start..label.span.end))
                    .with_message(&label.message)
                    .with_color(colours.next())
            })
            .collect();

        let primary = self
            .labels
            .first()
            .map(|label| label.span)
            .unwrap_or_else(|| SimpleSpan::from(0..0));

        Report::build(ReportKind::Error, (filename, primary.start..primary.end))
            .with_message(&self.message)
            .with_labels(labels)
            .finish()
            .eprint((filename, Source::from(source)))
            .unwrap();
    }
}

impl Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

// #[derive(Debug)]
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
}

impl Display for OpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidBinaryOp { op, left, right } => {
                write!(f, "cannot apply `{op}` to `{left}` and `{right}`")
            }
            Self::InvalidUnaryOp { op, expr } => {
                write!(f, "cannot apply `{op}` to `{expr}`")
            }
        }
    }
}
