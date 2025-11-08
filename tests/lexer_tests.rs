use frg::lexer::*;

#[test]
fn complex_declaration() {
    let input = "int target_frog = &frogs[0]";
    let output = vec![
        Token::Keyword(Keyword::Int),
        Token::Literal(Literal::Identifier("target_frog".into())),
        Token::Symbol(Symbol::Equals),
        Token::Symbol(Symbol::Ampersand),
        Token::Literal(Literal::Identifier("frogs".into())),
        Token::Symbol(Symbol::LeftBracket),
        Token::Literal(Literal::Number("0".into())),
        Token::Symbol(Symbol::RightBracket),
    ];
    let result = lex(input);
    assert_eq!(output, result);
    println!("{:?}", &result);
}

#[test]
fn simple_declaration() {
    let input = "int x = 5";
    let output = vec![
        Token::Keyword(Keyword::Int),
        Token::Literal(Literal::Identifier("x".into())),
        Token::Symbol(Symbol::Equals),
        Token::Literal(Literal::Number("5".into())),
    ];
    let result = lex(input);
    assert_eq!(output, result);
    println!("{:?}", &result);
}

#[test]
fn expression() {
    let input = "(5 + 2) * 21 / -1";
    let output = vec![
        Token::Symbol(Symbol::LeftParen),
        Token::Literal(Literal::Number("5".into())),
        Token::Symbol(Symbol::Plus),
        Token::Literal(Literal::Number("2".into())),
        Token::Symbol(Symbol::RightParen),
        Token::Symbol(Symbol::Star),
        Token::Literal(Literal::Number("21".into())),
        Token::Symbol(Symbol::FSlash),
        Token::Symbol(Symbol::Minus),
        Token::Literal(Literal::Number("1".into())),
    ];
    let result = lex(input);
    assert_eq!(output, result);
    println!("{:?}", &result);
}
