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
        just(Token::StringType),
        just(Token::ListType),
        just(Token::TypeType),
        just(Token::VoidType),
        just(Token::StructDecl),
        just(Token::If),
        just(Token::Return),
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

fn expr_parser<'src, I, EP, BP>(
    _src: &'src str,
    expr: EP,
    vtype: P<'src, I, (VarType<'src>, Spanned<&'src str>)>,
    block: BP,
) -> P<'src, I, Expr<'src>>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
    EP: Parser<'src, I, Expr<'src>, extra::Err<Rich<'src, Token<'src>>>> + Clone + 'src,
    BP: Parser<'src, I, Vec<Stmt<'src>>, extra::Err<Rich<'src, Token<'src>>>> + Clone + 'src,
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

    let type_literal = |type_token, type_vtype: VarType<'src>| {
        just(type_token)
            .map(move |_| Expression::TypeLiteral(Box::new(type_vtype.clone())))
            .spanned()
            .boxed()
    };
    let void_type = type_literal(Token::VoidType, VarType::Void);
    let int_type = type_literal(Token::IntType, VarType::Int);
    let float_type = type_literal(Token::FloatType, VarType::Float);
    let bool_type = type_literal(Token::BoolType, VarType::Bool);
    let string_type = type_literal(Token::StringType, VarType::Str);
    let type_type = type_literal(Token::TypeType, VarType::Type);
    let list_type = type_literal(Token::ListType, VarType::List);

    let bool_true: P<'src, I, Expression<'src>> = just(Token::BoolTrue)
        .map(|_| Expression::Literal(Literal::Bool(true)))
        .boxed();
    let bool_false: P<'src, I, Expression<'src>> = just(Token::BoolFalse)
        .map(|_| Expression::Literal(Literal::Bool(false)))
        .boxed();
    let bool_literal: P<'src, I, Expression<'src>> = choice((bool_true, bool_false)).boxed();
    let undefined = just(Token::Undefined)
        .map(|_| Expression::Literal(Literal::Undefined))
        .spanned()
        .boxed();

    // let struct_literal = expr
    //     .clone()
    //     .then(
    //         choice((
    //             name.clone()
    //                 .then_ignore(tok(Token::Equals))
    //                 .then(expr.clone())
    //                 .map(Some),
    //             tok(Token::Comma).map(|_| None),
    //         ))
    //         .repeated()
    //         .collect::<Vec<_>>()
    //         .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
    //         .delimited_by(left_brace.clone(), right_brace.clone()),
    //     )
    //     .map_with(|fields, e| Expression::StructLiteral {
    //         parent: Box::new(fields.0),
    //         fields: fields.1,
    //     })
    //     .labelled("struct literal")
    //     .as_context()
    //     .boxed();
    let struct_field = vtype
        .clone()
        .then(name.clone().spanned())
        .map(|((vtype, vtype_name), field_name)| (vtype, vtype_name, field_name.inner));
    let struct_fields = choice((struct_field.map(Some), tok(Token::Comma).map(|_| None)))
        .repeated()
        .at_least(1)
        .collect::<Vec<_>>()
        .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
        .delimited_by(left_brace.clone(), right_brace.clone());

    let struct_type_decl = tok(Token::StructDecl)
        .ignore_then(struct_fields)
        .map_with(|fields, e| Spanned {
            inner: Expression::StructTypeDeclaration { fields },
            span: e.span(),
        })
        .labelled("struct type")
        .as_context()
        .boxed();

    let function_literal = choice((name.clone().map(Some), tok(Token::Comma).map(|_| None)))
        .repeated()
        .collect::<Vec<_>>()
        .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
        .delimited_by(tok(Token::Pipe), tok(Token::Pipe))
        .then(block)
        .map_with(|(params, body), e| Spanned {
            inner: Expression::FunctionLiteral { params, body },
            span: e.span(),
        })
        .labelled("function literal")
        .as_context()
        .boxed();

    let vec_literal = choice((expr.clone().map(Some), tok(Token::Comma).map(|_| None)))
        .repeated()
        .collect::<Vec<_>>()
        .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
        .delimited_by(left_bracket.clone(), right_bracket.clone())
        .map_with(|values, e| Spanned {
            inner: Expression::VecLiteral(values),
            span: e.span(),
        })
        .labelled("vec literal")
        .as_context()
        .boxed();

    let atom = choice((
        struct_type_decl,
        function_literal,
        vec_literal,
        void_type,
        int_type,
        float_type,
        bool_type,
        string_type,
        type_type,
        list_type,
        undefined,
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

    let struct_literal_fields = choice((
        name.clone()
            .then_ignore(tok(Token::Equals))
            .then(expr.clone())
            .map(Some),
        tok(Token::Comma).map(|_| None),
    ))
    .repeated()
    .at_least(1)
    .collect::<Vec<_>>()
    .map(|items| items.into_iter().flatten().collect::<Vec<_>>())
    .delimited_by(left_brace, right_brace);

    let struct_literal = struct_literal_fields
        .map_with(|fields, e| {
            Postfix::StructLiteral(Spanned {
                inner: fields,
                span: e.span(),
            })
        })
        .labelled("struct literal")
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
                            parent: Box::new(base),
                            index: Box::new(index.inner),
                        },
                        span: SimpleSpan::from(start..index.span.end),
                    },
                    Postfix::Member(member) => Spanned {
                        inner: Expression::MemberAccess {
                            parent: Box::new(base),
                            member,
                        },
                        span: SimpleSpan::from(start..member.span.end),
                    },
                    Postfix::StructLiteral(fields) => Spanned {
                        inner: Expression::StructLiteral {
                            parent: Box::new(base),
                            fields: fields.inner,
                        },
                        span: SimpleSpan::from(start..fields.span.end),
                    },
                }
            },
        )
        .boxed();

    let unary_op = choice((
        tok(Token::Minus).to(UnaryOperator::Negative),
        tok(Token::Star).to(UnaryOperator::Dereference),
        tok(Token::Ampersand).to(UnaryOperator::Reference),
        tok(Token::Exclamation).to(UnaryOperator::Not),
    ))
    .map_with(|op, e| (op, e.span()))
    .boxed();

    let unary = unary_op
        .repeated()
        .foldr(postfix, |(op, op_span), rhs: Expr<'src>| {
            let span = SimpleSpan::from(op_span.start..rhs.span.end);
            Spanned {
                inner: Expression::UnaryOperation {
                    op,
                    right: Box::new(rhs),
                },
                span,
            }
        });
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

    comparison.labelled("expression").as_context().boxed()
}

fn type_parser<'src, I, EP>(
    src: &'src str,
    expr: EP,
) -> P<'src, I, (VarType<'src>, Spanned<&'src str>)>
where
    I: BorrowInput<'src, Token = Token<'src>, Span = SimpleSpan> + ValueInput<'src>,
    EP: Parser<'src, I, Expr<'src>, extra::Err<Rich<'src, Token<'src>>>> + 'src,
{
    expr.map(VarType::Custom)
        .map_with(move |vtype, e| {
            let span = e.span();
            (
                vtype,
                Spanned {
                    inner: &src[span.into_range()],
                    span,
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

    let mut expr_rec = Recursive::declare();
    let mut stmt_rec = Recursive::declare();

    let vtype = type_parser(src, expr_rec.clone());
    let block = stmt_rec
        .clone()
        .repeated()
        .collect::<Vec<Option<Stmt<'src>>>>()
        .map(|statements| statements.into_iter().flatten().collect::<Vec<_>>())
        .delimited_by(left_brace, right_brace)
        .labelled("block")
        .as_context()
        .boxed();

    let expr = expr_parser(src, expr_rec.clone(), vtype.clone(), block.clone());
    expr_rec.define(expr);

    let statement_body: P<'src, I, Option<Stmt<'src>>> = {
        let var_decl = vtype
            .then(name.spanned())
            .then(equals_assign.ignored().then(expr_rec.clone()).or_not())
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

        let var_assign = expr_rec
            .clone()
            .then(choice((
                tok(Token::Equals).to(AssignmentOperator::Equals),
                tok(Token::PlusEquals).to(AssignmentOperator::PlusEquals),
                tok(Token::MinusEquals).to(AssignmentOperator::MinusEquals),
                tok(Token::StarEquals).to(AssignmentOperator::TimesEquals),
                tok(Token::SlashEquals).to(AssignmentOperator::DivideEquals),
                tok(Token::PercentEquals).to(AssignmentOperator::ModulusEquals),
            )))
            .then(expr_rec.clone())
            .map(|((variable, op), expr)| Statement::VariableAssignment { variable, op, expr })
            .labelled("variable assignment")
            .as_context()
            .spanned()
            .boxed();

        let return_stmt = tok(Token::Return)
            .ignore_then(expr_rec.clone())
            .map(Statement::Return)
            .labelled("return statement")
            .as_context()
            .spanned()
            .boxed();

        let standalone_expr = expr_rec
            .clone()
            .map(Statement::StandaloneExpression)
            .spanned()
            .boxed();

        let block_stmt = block.clone().map(Statement::Block).spanned().boxed();

        let else_if = else_kw
            .clone()
            .ignore_then(if_kw.clone())
            .ignore_then(expr_rec.clone())
            .then(block.clone());
        let if_stmt = if_kw
            .ignore_then(expr_rec.clone())
            .then(block.clone())
            .then(else_if.repeated().collect::<Vec<_>>())
            .then(else_kw.ignore_then(block).or_not())
            .map(|(((condition, body), else_ifs), else_body)| Statement::If {
                condition,
                body,
                else_ifs,
                else_body: else_body.unwrap_or_default(),
            })
            .labelled("if statement")
            .as_context()
            .spanned()
            .boxed();

        choice((
            return_stmt,
            var_decl,
            var_assign,
            if_stmt,
            block_stmt,
            standalone_expr,
        ))
        .then_ignore(tok(Token::Semicolon).or_not())
        .map(Some)
        .labelled("statement")
        .as_context()
        .recover_with(via_parser(recovery_parser()))
        .boxed()
    }
    .boxed();
    stmt_rec.define(statement_body);

    stmt_rec
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
