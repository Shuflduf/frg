mod ast;
mod lexer;

fn main() {
    // let args = env::args().collect::<Vec<String>>();
    // if args.len() < 2 {
    //     eprintln!("Usage: frg <code>.frg");
    //     return;
    // }

    // let file_path = &args[1];
    // let input = fs::read_to_string("code.frog").unwrap();
    let input = "vec(vec(map(float, int))) numbers = 5";
    println!("Input: {input}");
    let tokens = lexer::lex(input);
    println!("Tokens: {tokens:?}");
    let tree = ast::parse(tokens);
    println!("AST: {tree:?}");

    let input = "vec(vec(float))";
    println!("Input: {input}");
    let tokens = lexer::lex(input);
    println!("Tokens: {tokens:?}");
    let var_type = ast::parse_type(None, &mut tokens.iter());
    println!("Type: {var_type:?}");
}

// #[cfg(tests)]
// mod tests {

//     #[test]
//     fn main_code_test() {

//     }
// }
