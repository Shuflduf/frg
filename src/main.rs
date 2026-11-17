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

fn parser<'src, I>() -> impl Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    let int_literal = select_ref! { Token::IntLiteral(n) => Expression::Literal(Literal::Int(*n)) };
    let float_literal =
        select_ref! { Token::FloatLiteral(n) => Expression::Literal(Literal::Float(*n)) };
    let identifier = select_ref! { Token::Identifier(name) => Expression::Identifier(name) };
    let name = select_ref! { Token::Identifier(name) => *name };
    let plus = just(Token::Plus).to(());
    let minus = just(Token::Minus).to(());
    let star = just(Token::Star).to(());
    let slash = just(Token::Slash).to(());
    let left_paren = just(Token::LeftParen).to(());
    let right_paren = just(Token::RightParen).to(());
    let left_bracket = just(Token::LeftBracket).to(());
    let right_bracket = just(Token::RightBracket).to(());
    let left_brace = just(Token::LeftBrace).to(());
    let right_brace = just(Token::RightBrace).to(());
    let if_kw = just(Token::If).to(());
    let else_kw = just(Token::Else).to(());
    let int_type = just(Token::IntType).map(|_| VarType::Int);
    let float_type = just(Token::FloatType).map(|_| VarType::Float);
    let bool_type = just(Token::BoolType).map(|_| VarType::Bool);
    let equals_assign = just(Token::Equals).to(());
    let equivalent = just(Token::Equivalent).to(());
    let not_equivalent = just(Token::NotEquivalent).to(());
    let bool_true = just(Token::BoolTrue).map(|_| Expression::Literal(Literal::Bool(true)));
    let bool_false = just(Token::BoolFalse).map(|_| Expression::Literal(Literal::Bool(false)));

    let bool_literal = choice((bool_true, bool_false));
    let vtype = choice((int_type, float_type, bool_type));

    let statement_start = choice((
        just(Token::IntType),
        just(Token::FloatType),
        just(Token::BoolType),
        just(Token::If),
    ))
    .ignored();
    let block_end = just(Token::RightBrace).ignored();
    let balanced_group = recursive(|balanced_group| {
        let closing = choice((
            just(Token::RightParen),
            just(Token::RightBracket),
            just(Token::RightBrace),
        ))
        .ignored();
        let contents = choice((balanced_group, any().and_is(closing.not()).ignored())).repeated();

        choice((
            contents
                .clone()
                .delimited_by(left_paren.clone(), right_paren.clone()),
            contents
                .clone()
                .delimited_by(left_bracket.clone(), right_bracket.clone()),
            contents.delimited_by(left_brace.clone(), right_brace.clone()),
        ))
        .ignored()
    });

    let recovery_first = choice((balanced_group.clone(), any().ignored()));
    let recovery_rest = choice((
        balanced_group,
        any()
            .and_is(statement_start.clone().or(block_end.clone()).not())
            .ignored(),
    ));
    let recovery = block_end
        .clone()
        .not()
        .ignore_then(recovery_first)
        .then(recovery_rest.repeated())
        .map(|_| None);

    let expr = recursive(|expr| {
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
    });

    let statement = recursive(|statement| {
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
            .recover_with(via_parser(recovery))
    });

    statement
        .repeated()
        .collect::<Vec<Option<Stmt<'src>>>>()
        .map(|statements| statements.into_iter().flatten().collect::<Vec<_>>())

    // let int = text::int(10).map(|s: &str| Expr::Int(s.parse().unwrap()));

    // let atom = int.or(expr.delimited_by(just('('), just(')'))).padded();

    // let op = |c| just(c).padded();

    // let unary = op("-")
    //     .repeated()
    //     .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

    // let product = unary.clone().foldl(
    //     choice((
    //         op("*").to(Expr::Mul as fn(_, _) -> _),
    //         op("/").to(Expr::Div as fn(_, _) -> _),
    //     ))
    //     .then(unary)
    //     .repeated(),
    //     |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    // );

    // #[allow(clippy::let_and_return)]
    // let sum = product.clone().foldl(
    //     choice((
    //         op("+").to(Expr::Add as fn(_, _) -> _),
    //         op("-").to(Expr::Sub as fn(_, _) -> _),
    //     ))
    //     .then(product)
    //     .repeated(),
    //     |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
    // );

    // sum
    // })
}

// fn line_col(src: &str, span: SimpleSpan) -> (usize, usize) {
//     let before = &src[..span.start];

//     let line = before.bytes().filter(|&b| b == b'\n').count() + 1;
//     let col = before
//         .rsplit('\n')
//         .next()
//         .map_or(1, |s| s.chars().count() + 1);
// }

#[cfg(test)]
mod tests {
    // use super::*;

    // #[test]
    // fn add() {
    //     let src = "6  +  4  ";
    //     assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), 10);
    // }

    // #[test]
    // fn sub() {
    //     let src = "-6  -  4 +13 - 95 ";
    //     assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -92);
    // }

    // #[test]
    // fn mul() {
    //     let src = "-6  -  4 * 7 +13 - 95 ";
    //     assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -116);
    // }
    // #[test]
    // fn div() {
    //     let src = "-6 / 3  -  4 * 7 +13 - 95 / 5 ";
    //     assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -36);
    // }
    // #[test]
    // fn paren() {
    //     let src = "-6 / 3  -  4 * (7 +13) - 95 / 5 ";
    //     assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -101);
    // }
}
