use ariadne::{ColorGenerator, Label, Source};
use chumsky::{input::BorrowInput, prelude::*};
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
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan>,
{
    let int_literal = select_ref! { Token::IntLiteral(n) => Expression::Literal(Literal::Int(*n)) };
    let float_literal =
        select_ref! { Token::FloatLiteral(n) => Expression::Literal(Literal::Float(*n)) };
    let identifier = select_ref! { Token::Identifier(name) => Expression::Identifier(name) };
    let left_paren = select_ref! { Token::LeftParen => () };
    let right_paren = select_ref! { Token::RightParen => () };
    let plus = select_ref! { Token::Plus => () };
    let minus = select_ref! { Token::Minus => () };
    let star = select_ref! { Token::Star => () };
    let slash = select_ref! { Token::Slash => () };
    let left_brace = select_ref! { Token::LeftBrace => () };
    let right_brace = select_ref! { Token::RightBrace => () };
    let if_kw = select_ref! { Token::If => () };
    let else_kw = select_ref! { Token::Else => () };
    let int_type = select_ref! { Token::IntType => VarType::Int };
    let float_type = select_ref! { Token::FloatType => VarType::Float };
    let bool_type = select_ref! { Token::BoolType => VarType::Bool };
    let name = select_ref! { Token::Identifier(name) => *name };
    let equals_assign = select_ref! { Token::Equals => () };
    let equivalent = select_ref! { Token::Equivalent => () };
    let not_equivalent = select_ref! { Token::NotEquivalent => () };
    let bool_true = select_ref! { Token::BoolTrue => Expression::Literal(Literal::Bool(true)) };
    let bool_false = select_ref! { Token::BoolFalse => Expression::Literal(Literal::Bool(false)) };

    let bool_literal = choice((bool_true, bool_false));

    let expr = recursive(|expr| {
        let atom = choice((
            int_literal.spanned(),
            float_literal.spanned(),
            bool_literal.spanned(),
            identifier.spanned(),
            expr.delimited_by(left_paren, right_paren),
        ));
        let unary = minus.to_span().repeated().foldr(
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

    let vtype = choice((int_type, float_type, bool_type));

    let statement = recursive(|statement| {
        let block = statement
            .clone()
            .repeated()
            .collect()
            .delimited_by(left_brace, right_brace)
            .boxed();

        let var_decl = vtype
            .then(name.spanned())
            .then_ignore(equals_assign)
            .then(expr.clone())
            .map(|((vtype, name), value)| Statement::VariableDeclaration { vtype, name, value })
            .spanned()
            .boxed();

        let else_if = else_kw
            .ignore_then(if_kw)
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
            .spanned()
            .boxed();

        choice((var_decl, if_stmt))
    });

    statement.repeated().collect()

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
    use super::*;

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
