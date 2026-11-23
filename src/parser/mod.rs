use chumsky::{
    input::{BorrowInput, ValueInput},
    prelude::*,
};

use crate::{
    ast::{
        AssignmentOperator, BinaryOperator, Expr, Expression, Literal, Statement, Stmt,
        UnaryOperator, VarType,
    },
    lexer::Token,
};

type P<'src, I, O> = Boxed<'src, 'src, I, O, extra::Err<Rich<'src, Token<'src>>>>;

enum Postfix<'src> {
    Call(Spanned<Vec<Expr<'src>>>),
    Index(Spanned<Expr<'src>>),
    Member(Spanned<&'src str>),
    StructLiteral(Spanned<Vec<(&'src str, Expr<'src>)>>),
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

fn expr_parser<'src, I, EP>(
    _src: &'src str,
    expr: EP,
    vtype: P<'src, I, (VarType<'src>, Spanned<&'src str>)>,
) -> P<'src, I, Expr<'src>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
    EP: Parser<'src, I, Expr<'src>, extra::Err<Rich<'src, Token<'src>>>> + Clone + 'src,
{
    let int_literal =
        select_ref! { Token::IntLiteral(n) => Expression::Literal(Literal::Int(*n)) }.boxed();
    let float_literal =
        select_ref! { Token::FloatLiteral(n) => Expression::Literal(Literal::Float(*n)) }.boxed();
    let str_literal =
        select_ref! { Token::StringLiteral(s) => Expression::Literal(Literal::Str(s.to_string())) }
            .boxed();
    let identifier =
        select_ref! { Token::Identifier(name) => Expression::Identifier(name) }.boxed();
    let name = select_ref! { Token::Identifier(name) => *name }.boxed();
    let plus = tok(Token::Plus);
    let minus = tok(Token::Minus);
    let star = tok(Token::Star);
    let slash = tok(Token::Slash);
    let left_paren = tok(Token::LeftParen);
    let right_paren = tok(Token::RightParen);
    let left_bracket = tok(Token::LeftBracket);
    let right_bracket = tok(Token::RightBracket);
    let left_brace = tok(Token::LeftBrace);
    let right_brace = tok(Token::RightBrace);
    let bool_true: P<'src, I, Expression<'src>> = just(Token::BoolTrue)
        .map(|_| Expression::Literal(Literal::Bool(true)))
        .boxed();
    let bool_false: P<'src, I, Expression<'src>> = just(Token::BoolFalse)
        .map(|_| Expression::Literal(Literal::Bool(false)))
        .boxed();
    let bool_literal: P<'src, I, Expression<'src>> = choice((bool_true, bool_false)).boxed();

    let atom = choice((
        int_literal.spanned(),
        float_literal.spanned(),
        bool_literal.spanned(),
        str_literal.spanned(),
        identifier.spanned(),
        expr.clone()
            .delimited_by(left_paren.clone(), right_paren.clone()),
    ));
    let func_args = choice((
        expr.clone().map(Some),
        tok(Token::Comma).map(|_| None::<Expr<'src>>),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
    .delimited_by(left_paren, right_paren)
    .map_with(|params, e| {
        Postfix::Call(Spanned {
            inner: params,
            span: e.span(),
        })
    })
    .labelled("function call")
    .as_context()
    .boxed();

    let index = expr
        .clone()
        .delimited_by(left_bracket, right_bracket)
        .map_with(|index, e| {
            Postfix::Index(Spanned {
                inner: index,
                span: e.span(),
            })
        })
        .labelled("index access")
        .as_context()
        .boxed();

    let member = tok(Token::Period)
        .ignore_then(name.clone().map_with(|name, e| Spanned {
            inner: name,
            span: e.span(),
        }))
        .map(Postfix::Member)
        .labelled("member access")
        .as_context()
        .boxed();

    let struct_literal = choice((
        name.clone()
            .then_ignore(tok(Token::Equals))
            .then(expr.clone())
            .map(Some),
        tok(Token::Comma).map(|_| None),
    ))
    .repeated()
    .collect::<Vec<_>>()
    .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
    .delimited_by(left_brace.clone(), right_brace.clone())
    .map_with(|fields, e| {
        Postfix::StructLiteral(Spanned {
            inner: fields,
            span: e.span(),
        })
    })
    .labelled("struct literal")
    .as_context()
    .boxed();

    let postfix = atom
        .foldl(
            choice((func_args, index, member, struct_literal)).repeated(),
            |base, op| {
                let start = base.span.start;
                match op {
                    Postfix::Call(params) => Spanned {
                        inner: Expression::FunctionCall {
                            function: Box::new(base),
                            params: params.inner,
                        },
                        span: SimpleSpan::from(start..params.span.end),
                    },
                    Postfix::Index(index) => Spanned {
                        inner: Expression::IndexAccess {
                            expr: Box::new(base),
                            index: Box::new(index.inner),
                        },
                        span: SimpleSpan::from(start..index.span.end),
                    },
                    Postfix::Member(member) => Spanned {
                        inner: Expression::MemberAccess {
                            expr: Box::new(base),
                            member: member,
                        },
                        span: SimpleSpan::from(start..member.span.end),
                    },
                    Postfix::StructLiteral(fields) => Spanned {
                        inner: Expression::StructLiteral {
                            expr: Box::new(base),
                            fields: fields.inner,
                        },
                        span: SimpleSpan::from(start..fields.span.end),
                    },
                }
            },
        )
        .boxed();

    let unary = minus.clone().to_span().repeated().foldr(
        postfix,
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
            tok(Token::Equivalent).to(BinaryOperator::Equals),
            tok(Token::NotEquivalent).to(BinaryOperator::NotEquals),
            tok(Token::LeftAngle).to(BinaryOperator::LessThan),
            tok(Token::RightAngle).to(BinaryOperator::GreaterThan),
            tok(Token::LessThanOrEqual).to(BinaryOperator::LessThanOrEqual),
            tok(Token::GreaterThanOrEqual).to(BinaryOperator::GreaterThanOrEqual),
        ))
        .then(sum)
        .repeated(),
        |lhs, (op, rhs)| binary(lhs, op, rhs),
    );

    let boring_expr = comparison.labelled("expression").as_context().boxed();

    let struct_type_decl = tok(Token::StructDecl)
        .ignore_then(
            choice((
                vtype.then(name.clone()).map(Some),
                tok(Token::Comma).map(|_| None),
            ))
            .repeated()
            .collect::<Vec<_>>()
            .map(|items| {
                items
                    .into_iter()
                    .flatten()
                    .map(|((vtype, vtype_name), field_name)| (vtype, vtype_name, field_name))
                    .collect::<Vec<_>>()
            })
            .delimited_by(left_brace.clone(), right_brace.clone()),
        )
        .map_with(|fields, e| Spanned {
            inner: Expression::StructTypeDeclaration { fields },
            span: e.span(),
        })
        .labelled("struct type")
        .as_context()
        .boxed();

    choice((struct_type_decl, boring_expr)).boxed()
}

fn type_parser<'src, I, EP>(
    src: &'src str,
    expr: EP,
) -> P<'src, I, (VarType<'src>, Spanned<&'src str>)>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
    EP: Parser<'src, I, Expr<'src>, extra::Err<Rich<'src, Token<'src>>>> + 'src,
{
    let vtype_token: P<'src, I, VarType> = choice((
        just(Token::IntType).to(VarType::Int),
        just(Token::FloatType).to(VarType::Float),
        just(Token::BoolType).to(VarType::Bool),
        just(Token::StringType).to(VarType::Str),
        just(Token::TypeType).to(VarType::Type),
        expr.map(VarType::Custom),
    ))
    .boxed();

    vtype_token
        .map_with(move |vtype, e| {
            let span = e.span();
            (
                vtype,
                Spanned {
                    inner: &src[span.into_range()],
                    span: span,
                },
            )
        })
        .boxed()
}

fn statement_parser<'src, I>(
    src: &'src str,
) -> impl Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    let left_brace = tok(Token::LeftBrace);
    let right_brace = tok(Token::RightBrace);
    let name = select_ref! { Token::Identifier(name) => *name }.boxed();
    let equals_assign = tok(Token::Equals);
    let if_kw = tok(Token::If);
    let else_kw = tok(Token::Else);

    let mut rec = Recursive::declare();
    let vtype = type_parser(src, rec.clone());
    let expr = expr_parser(src, rec.clone(), vtype.clone());
    rec.define(expr);

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
            .then(equals_assign.ignored().then(rec.clone()).or_not())
            .map(
                |((decl_vtype, decl_name), decl_value)| Statement::VariableDeclaration {
                    vtype_name: decl_vtype.1,
                    vtype: decl_vtype.0,
                    name: decl_name,
                    value: decl_value.map(|s| s.1),
                },
            )
            .labelled("variable declaration")
            .as_context()
            .spanned()
            .boxed();

        let var_assign = rec
            .clone()
            .then(choice((
                tok(Token::Equals).to(AssignmentOperator::Equals),
                tok(Token::PlusEquals).to(AssignmentOperator::PlusEquals),
                tok(Token::MinusEquals).to(AssignmentOperator::MinusEquals),
                tok(Token::StarEquals).to(AssignmentOperator::TimesEquals),
                tok(Token::SlashEquals).to(AssignmentOperator::DivideEquals),
                tok(Token::PercentEquals).to(AssignmentOperator::ModulusEquals),
            )))
            .then(rec.clone())
            .map(|((variable, op), expr)| Statement::VariableAssignment { variable, op, expr })
            .labelled("variable assignment")
            .as_context()
            .spanned()
            .boxed();

        let else_if = else_kw
            .clone()
            .ignore_then(if_kw.clone())
            .ignore_then(rec.clone())
            .then(block.clone());
        let if_stmt = if_kw
            .ignore_then(rec.clone())
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

        choice((var_decl, var_assign, if_stmt))
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

pub fn create_parser<'src, I>(
    src: &'src str,
) -> impl Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
{
    statement_parser(src)
}
