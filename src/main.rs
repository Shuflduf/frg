use std::fs;

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
    let input = "struct Frog { int legs, bool alive }";
    println!("Input: {input}");
    let tokens = lexer::lex(input);
    println!("Tokens: {tokens:?}");
    let tree = ast::parse(tokens);
    println!("AST: {tree:?}");
}

// #[cfg(tests)]
// mod tests {

//     #[test]
//     fn main_code_test() {

//     }
// }
