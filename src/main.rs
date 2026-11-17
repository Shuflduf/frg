use ariadne::{ColorGenerator, Label, Source};
use chumsky::{
    input::{BorrowInput, ValueInput},
    prelude::*,
};
use clap::Parser as _;

use crate::{
    ast::{BinaryOperator, Expr, Expression, Literal, Statement, Stmt, UnaryOperator, VarType},
    tokens::Token,
};

mod ast;
mod cli;
mod errors;
mod interpreter;
mod tokens;

type P<'src, I, O> = Boxed<'src, 'src, I, O, extra::Err<Rich<'src, Token<'src>>>>;

fn main() {
    let args = cli::FrgArgs::parse();
    match args {
        cli::FrgArgs::Build(_build_args) => println!("build"),
        cli::FrgArgs::Run(_run_args) => println!("run"),
        cli::FrgArgs::Test => test(),
    }
}

fn test() {
    let src = "int thing = 8 / 3.0";
    let tokens = tokens::lex(src);
    // println!("{tokens:?}");
    let eoi = SimpleSpan::from(src.len()..src.len());
    let input = tokens.as_slice().split_spanned(eoi);
    match parser().parse(input).into_result() {
        Ok(ast) => {
            // println!("{ast:#?}");
            match interpreter::run(&ast) {
                Ok(output) => println!("{output}"),
                Err(eval_err) => eval_err.report(src, "tester"),
            };
        }
        Err(parse_errs) => {
            let mut colours = ColorGenerator::new();
            let labels: Vec<Label<(&str, std::ops::Range<usize>)>> = parse_errs
                .iter()
                .map(|e| {
                    Label::new(("tester", e.span().start..e.span().end))
                        .with_message(format!("{:?}", e))
                        .with_color(colours.next())
                })
                .collect();
            ariadne::Report::build(ariadne::ReportKind::Error, ("tester", eoi.into_range()))
                .with_message("failed to parse")
                .with_labels(labels)
                .finish()
                .eprint(("tester", Source::from(src)))
                .unwrap();
        }
    }
}

fn binary<'src>(lhs: Expr<'src>, op: BinaryOperator, rhs: Expr<'src>) -> Expr<'src> {
    let span = SimpleSpan::from(lhs.span.start..rhs.span.end);
    Spanned {
        inner: Expression::BinaryOperation {
            left: Box::new(lhs),
            op,
            right: Box::new(rhs),
        },
        span,
    }
}

fn recovery_parser<'src, I>() -> P<'src, I, Option<Stmt<'src>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    let statement_start = choice((
        just(Token::IntType),
        just(Token::FloatType),
        just(Token::BoolType),
        just(Token::If),
    ))
    .ignored()
    .boxed();
    let block_end = just(Token::RightBrace).ignored().boxed();

    let balanced_group: P<'src, I, ()> = recursive(|balanced_group| {
        let closing = choice((
            just(Token::RightParen),
            just(Token::RightBracket),
            just(Token::RightBrace),
        ))
        .ignored();
        let contents = choice((balanced_group, any().and_is(closing.not()).ignored())).repeated();

        choice((
            contents.clone().delimited_by(
                tok(Token::LeftParen).clone(),
                tok(Token::RightParen).clone(),
            ),
            contents.clone().delimited_by(
                tok(Token::LeftBracket).clone(),
                tok(Token::RightBracket).clone(),
            ),
            contents.delimited_by(
                tok(Token::LeftBrace).clone(),
                tok(Token::RightBrace).clone(),
            ),
        ))
        .ignored()
        .boxed()
    })
    .boxed();

    let recovery_first = choice((balanced_group.clone(), any().ignored())).boxed();
    let recovery_rest = choice((
        balanced_group,
        any()
            .and_is(statement_start.clone().or(block_end.clone()).not())
            .ignored(),
    ))
    .boxed();
    block_end
        .clone()
        .not()
        .ignore_then(recovery_first)
        .then(recovery_rest.repeated())
        .map(|_| None)
        .boxed()
}

fn tok<'src, I>(token: Token<'src>) -> P<'src, I, ()>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    just(token).to(()).boxed()
}

fn expr_parser<'src, I>() -> P<'src, I, Expr<'src>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    let int_literal: P<'src, I, Expression<'src>> =
        select_ref! { Token::IntLiteral(n) => Expression::Literal(Literal::Int(*n)) }.boxed();
    let float_literal: P<'src, I, Expression<'src>> =
        select_ref! { Token::FloatLiteral(n) => Expression::Literal(Literal::Float(*n)) }.boxed();
    let identifier: P<'src, I, Expression<'src>> =
        select_ref! { Token::Identifier(name) => Expression::Identifier(name) }.boxed();
    let plus = tok(Token::Plus);
    let minus = tok(Token::Minus);
    let star = tok(Token::Star);
    let slash = tok(Token::Slash);
    let left_paren = tok(Token::LeftParen);
    let right_paren = tok(Token::RightParen);
    let equivalent = tok(Token::Equivalent);
    let not_equivalent = tok(Token::NotEquivalent);
    let bool_true: P<'src, I, Expression<'src>> = just(Token::BoolTrue)
        .map(|_| Expression::Literal(Literal::Bool(true)))
        .boxed();
    let bool_false: P<'src, I, Expression<'src>> = just(Token::BoolFalse)
        .map(|_| Expression::Literal(Literal::Bool(false)))
        .boxed();
    let bool_literal: P<'src, I, Expression<'src>> = choice((bool_true, bool_false)).boxed();

    recursive(|expr| {
        let atom = choice((
            int_literal.spanned(),
            float_literal.spanned(),
            bool_literal.spanned(),
            identifier.spanned(),
            expr.delimited_by(left_paren, right_paren),
        ));
        let unary = minus.clone().to_span().repeated().foldr(
            atom,
            |op_span: SimpleSpan, rhs: Spanned<Expression<'_>>| {
                let span = SimpleSpan::from(op_span.start..rhs.span.end);
                Spanned {
                    inner: Expression::UnaryOperation {
                        op: UnaryOperator::Negative,
                        expr: Box::new(rhs),
                    },
                    span,
                }
            },
        );
        let product = unary.clone().foldl(
            choice((
                star.to(BinaryOperator::Multiply),
                slash.to(BinaryOperator::Divide),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| binary(lhs, op, rhs),
        );
        let sum = product.clone().foldl(
            choice((
                plus.to(BinaryOperator::Add),
                minus.to(BinaryOperator::Subtract),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| binary(lhs, op, rhs),
        );
        let comparison = sum.clone().foldl(
            choice((
                equivalent.to(BinaryOperator::Equals),
                not_equivalent.to(BinaryOperator::NotEquals),
            ))
            .then(sum)
            .repeated(),
            |lhs, (op, rhs)| binary(lhs, op, rhs),
        );

        comparison.boxed()
    })
    .boxed()
}

fn statement_parser<'src, I>()
-> impl Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    let left_brace = tok(Token::LeftBrace);
    let right_brace = tok(Token::RightBrace);
    let int_type: P<'src, I, VarType> = just(Token::IntType).map(|_| VarType::Int).boxed();
    let float_type: P<'src, I, VarType> = just(Token::FloatType).map(|_| VarType::Float).boxed();
    let bool_type: P<'src, I, VarType> = just(Token::BoolType).map(|_| VarType::Bool).boxed();
    let vtype: P<'src, I, VarType> = choice((int_type, float_type, bool_type)).boxed();
    let name: P<'src, I, &'src str> = select_ref! { Token::Identifier(name) => *name }.boxed();
    let equals_assign = tok(Token::Equals);
    let if_kw = tok(Token::If);
    let else_kw = tok(Token::Else);

    let expr = expr_parser();

    let statement: P<'src, I, Option<Stmt<'src>>> = recursive(|statement| {
        let block = statement
            .clone()
            .repeated()
            .collect::<Vec<Option<Stmt<'src>>>>()
            .map(|statements| statements.into_iter().flatten().collect::<Vec<_>>())
            .delimited_by(left_brace, right_brace)
            .labelled("block")
            .as_context()
            .boxed();

        let var_decl = vtype
            .then(name.spanned())
            .then_ignore(equals_assign)
            .then(expr.clone())
            .map(|((vtype, name), value)| Statement::VariableDeclaration { vtype, name, value })
            .labelled("variable declaration")
            .as_context()
            .spanned()
            .boxed();

        let else_if = else_kw
            .clone()
            .ignore_then(if_kw.clone())
            .ignore_then(expr.clone())
            .then(block.clone());
        let if_stmt = if_kw
            .ignore_then(expr.clone())
            .then(block.clone())
            .then(else_if.repeated().collect::<Vec<_>>())
            .then(else_kw.ignore_then(block).or_not())
            .map(
                |(((condition, body), else_ifs), else_body)| Statement::IfStatement {
                    condition,
                    body,
                    else_ifs,
                    else_body: else_body.unwrap_or_default(),
                },
            )
            .labelled("if statement")
            .as_context()
            .spanned()
            .boxed();

        choice((var_decl, if_stmt))
            .map(Some)
            .labelled("statement")
            .as_context()
            .recover_with(via_parser(recovery_parser()))
            .boxed()
    })
    .boxed();

    statement
        .repeated()
        .collect::<Vec<Option<Stmt<'src>>>>()
        .map(|statements| statements.into_iter().flatten().collect::<Vec<_>>())
}

fn parser<'src, I>() -> impl Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    statement_parser()
}
