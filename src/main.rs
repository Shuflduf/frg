use chumsky::prelude::*;
use clap::Parser as _;

mod ast;
mod cli;

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
    let src = "-59 + 3842 - (984 +4)*2";
    // println!("{:?}", parser().parse("59"));
    match parser().parse(src).into_result() {
        Ok(ast) => match eval(&ast) {
            Ok(output) => println!("{output}"),
            Err(eval_err) => eprintln!("Eval Error: {eval_err}"),
        },
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| eprintln!("Parse error: {e}")),
    }
}

fn parser<'src>() -> impl Parser<'src, &'src str, Expr<'src>> {
    recursive(|expr| {
        let int = text::int(10).map(|s: &str| Expr::Int(s.parse().unwrap()));

        let atom = int.or(expr.delimited_by(just('('), just(')'))).padded();

        let op = |c| just(c).padded();

        let unary = op("-")
            .repeated()
            .foldr(atom, |_op, rhs| Expr::Neg(Box::new(rhs)));

        let product = unary.clone().foldl(
            choice((
                op("*").to(Expr::Mul as fn(_, _) -> _),
                op("/").to(Expr::Div as fn(_, _) -> _),
            ))
            .then(unary)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );
        #[allow(clippy::let_and_return)]
        let sum = product.clone().foldl(
            choice((
                op("+").to(Expr::Add as fn(_, _) -> _),
                op("-").to(Expr::Sub as fn(_, _) -> _),
            ))
            .then(product)
            .repeated(),
            |lhs, (op, rhs)| op(Box::new(lhs), Box::new(rhs)),
        );

        sum
    })
}

fn eval<'src>(expr: &'src Expr<'src>) -> Result<i32, String> {
    match expr {
        Expr::Int(x) => Ok(*x),
        Expr::Neg(a) => Ok(-eval(a)?),
        Expr::Add(a, b) => Ok(eval(a)? + eval(b)?),
        Expr::Sub(a, b) => Ok(eval(a)? - eval(b)?),
        Expr::Mul(a, b) => Ok(eval(a)? * eval(b)?),
        Expr::Div(a, b) => Ok(eval(a)? / eval(b)?),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let src = "6  +  4  ";
        assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), 10);
    }

    #[test]
    fn sub() {
        let src = "-6  -  4 +13 - 95 ";
        assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -92);
    }

    #[test]
    fn mul() {
        let src = "-6  -  4 * 7 +13 - 95 ";
        assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -116);
    }
    #[test]
    fn div() {
        let src = "-6 / 3  -  4 * 7 +13 - 95 / 5 ";
        assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -36);
    }
    #[test]
    fn paren() {
        let src = "-6 / 3  -  4 * (7 +13) - 95 / 5 ";
        assert_eq!(eval(&parser().parse(src).unwrap()).unwrap(), -101);
    }
}
