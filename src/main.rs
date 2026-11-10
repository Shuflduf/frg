use chumsky::prelude::*;
use clap::Parser as _;

use crate::{
    ast::{BinaryOperator, Expression, Literal, Statement, UnaryOperator, VarType},
    tokens::{Token, TokenData},
};

mod ast;
mod cli;
mod interpreter;
mod tokens;

#[allow(dead_code)]
#[derive(Debug)]
enum Expr<'src> {
    Int(i32),
    Var(&'src str),

    Neg(Box<Expr<'src>>),
    Add(Box<Expr<'src>>, Box<Expr<'src>>),
    Sub(Box<Expr<'src>>, Box<Expr<'src>>),
    Mul(Box<Expr<'src>>, Box<Expr<'src>>),
    Div(Box<Expr<'src>>, Box<Expr<'src>>),
}

fn main() {
    let args = cli::FrgArgs::parse();
    match args {
        cli::FrgArgs::Build(_build_args) => println!("build"),
        cli::FrgArgs::Run(_run_args) => println!("run"),
        cli::FrgArgs::Test => test(),
    }
}

fn test() {
    let src = "float test = 59 + 26.0";
    let tokens = tokens::lex(src);
    println!("{tokens:?}");
    match parser().parse(&tokens).into_result() {
        Ok(ast) => match interpreter::run(&ast) {
            Ok(output) => println!("{output}"),
            Err(eval_err) => eprintln!("Eval Error: {eval_err}"),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| eprintln!("Parse error: {e}")),
    }
}

fn parser<'src>() -> impl Parser<'src, &'src [TokenData<'src>], Vec<Statement<'src>>> {
    // recursive(|expr| {
    let int_literal = select_ref! {
        TokenData { token: Token::IntLiteral(n), .. } => Expression::Literal(Literal::Int(*n))
    };
    let float_literal = select_ref! {
        TokenData { token: Token::FloatLiteral(n), .. } => Expression::Literal(Literal::Float(*n))
    };
    let identifier = select_ref! {
        TokenData { token: Token::Identifier(name), .. } => Expression::Identifier(name)
    };
    let left_paren = select_ref! {TokenData { token: Token::LeftParen, .. } => () };
    let right_paren = select_ref! {TokenData { token: Token::RightParen, .. } => () };
    let plus = select_ref! {TokenData { token: Token::Plus, .. } => () };
    let minus = select_ref! {TokenData { token: Token::Minus, .. } => () };
    let star = select_ref! {TokenData { token: Token::Star, .. } => () };
    let slash = select_ref! {TokenData { token: Token::Slash, .. } => () };

    let expr = recursive(|expr| {
        let atom = choice((
            int_literal,
            float_literal,
            identifier,
            expr.delimited_by(left_paren, right_paren),
        ));
        let unary = minus
            .clone()
            .repeated()
            .foldr(atom, |_op, rhs| Expression::UnaryOperation {
                op: UnaryOperator::Negative,
                expr: Box::new(rhs),
            });
        let product = unary.clone().foldl(
            choice((
                star.to(BinaryOperator::Multiply),
                slash.to(BinaryOperator::Divide),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| Expression::BinaryOperation {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            },
        );
        let sum = product.clone().foldl(
            choice((
                plus.to(BinaryOperator::Add),
                minus.to(BinaryOperator::Subtract),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| Expression::BinaryOperation {
                left: Box::new(lhs),
                op,
                right: Box::new(rhs),
            },
        );

        sum
    });
    let int_type = select_ref! {
        TokenData { token: Token::IntType, .. } => VarType::Int
    };
    let float_type = select_ref! {
        TokenData { token: Token::FloatType, .. } => VarType::Float
    };
    let name = select_ref! {
        TokenData { token: Token::Identifier(name), .. } => *name
    };
    let equals = select_ref! {
        TokenData { token: Token::Equals, .. } => ()
    };

    let vtype = choice((int_type, float_type));

    vtype
        .then(name)
        .then_ignore(equals)
        .then(expr)
        .map(|((vtype, name), value)| Statement::VariableDeclaration { vtype, name, value })
        .repeated()
        .collect()
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

fn eval<'src>(program: &Vec<Statement<'src>>) -> Result<i32, String> {
    // let mut state = println!("{program:?}");
    // match expr {
    //     Expr::Int(x) => Ok(*x),
    //     Expr::Neg(a) => Ok(-eval(a)?),
    //     Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
    //     Expr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
    //     Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
    //     Expr::Div(a, b) => Ok(eval(a)? / eval(b)?),
    //     _ => todo!(),
    // }
    Ok(0)
}

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
