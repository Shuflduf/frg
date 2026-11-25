use std::{fs, time::Instant};

use chumsky::prelude::*;
use clap::Parser as _;

use crate::{
    cli::RunArgs,
    errors::{Diagnostic, check_delimiters},
};

mod ast;
mod cli;
mod errors;
mod interpreter;
mod lexer;
mod parser;

fn main() {
    let args = cli::FrgArgs::parse();
    match args {
        cli::FrgArgs::Build(_build_args) => println!("build"),
        cli::FrgArgs::Run(args) => run(args),
        cli::FrgArgs::Test => test(),
    }
}

fn run(args: RunArgs) {
    // TODO: proper error reporting
    let filename = args.file.file_name().expect("Invalid filename");
    let src = fs::read_to_string(&args.file).expect("Failed to read from file");
    println!("{filename:?} {src:?}");
    execute(&src, filename.to_str().unwrap());
}

fn test() {
    let src = r#"
void(int) test = |val| {
    if val == 1 {
        fuck = 3.14
    }   
}
    "#;
    let filename = "tester.frg";
    execute(src, filename);
}

fn execute(src: &str, filename: &str) {
    let now = Instant::now();

    let tokens = match lexer::lex(src) {
        Ok(t) => t,
        Err(diag) => {
            diag.report(src, filename);
            return;
        }
    };
    println!("Lexing: {:.5}s", now.elapsed().as_secs_f64());
    // println!("{tokens:?}");
    let eoi = SimpleSpan::from(src.len()..src.len());
    let delimiter_check = check_delimiters(&tokens);
    println!("Check #1: {:.5}s", now.elapsed().as_secs_f64());
    let can_parse = delimiter_check.can_parse;
    let mut diags = delimiter_check.diagnostics;
    let delimiter_spans: Vec<SimpleSpan> =
        diags.iter().filter_map(Diagnostic::primary_span).collect();
    if !can_parse {
        report_all(&diags, src, filename);
        return;
    }
    let parse_tokens: Vec<_> = tokens
        .iter()
        .filter(|token| !delimiter_spans.contains(&token.span))
        .cloned()
        .collect();
    let input = parse_tokens.as_slice().split_spanned(eoi);
    let parser = parser::create_parser(src);
    println!("Create parser: {:.5}s", now.elapsed().as_secs_f64());
    let ast = match parser.parse(input).into_result() {
        Ok(ast) => {
            if diags.is_empty() {
                ast
            } else {
                report_all(&diags, src, filename);
                return;
            }
        }
        Err(parse_errs) => {
            for error in parse_errs {
                let diag = Diagnostic::from_parse_error(&error, &tokens, src);

                if diag
                    .primary_span()
                    .is_some_and(|span| delimiter_spans.contains(&span))
                {
                    continue;
                }

                diags.push(diag);
            }
            report_all(&diags, src, filename);
            return;
        }
    };
    println!("Parse: {:.5}s", now.elapsed().as_secs_f64());
    // println!("{ast:#?}");
    match interpreter::run(&ast) {
        Ok(output) => println!("{output}"),
        Err(eval_err) => eval_err.report(src, filename),
    };
    println!("Interpreter: {:.5}s", now.elapsed().as_secs_f64());
}

fn report_all(diags: &[Diagnostic], src: &str, filename: &str) {
    let mut diags: Vec<_> = diags.iter().collect();
    diags.sort_by_key(|diag| diag.primary_span().map_or(usize::MAX, |span| span.start));
    for diag in diags {
        diag.report(src, filename);
    }
}
