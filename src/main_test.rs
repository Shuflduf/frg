use std::{env, fs};

mod ast;
mod interpreter;
mod lexer;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    // if args.len() < 2 {
    //     eprintln!("Usage: frg <code>.frg");
    //     return;
    // }

    let file_path = &args.get(1).map_or("test.frg", |v| v);
    let input = fs::read_to_string(file_path).unwrap();

    // let input = "Frog rocket = { legs: 4 + 9, name: \"Josch\" } int number = 5";
    // let input = "int number = 5";
    println!("Input: {input}");
    let tokens = lexer::lex(&input);
    println!("Tokens: {tokens:?}");
    let tree = ast::parse(tokens);
    println!("AST: {tree:#?}");
    println!("Output:");
    let return_value = interpreter::interpret(tree);
    println!("Returned: {return_value:?}")
}

// #[cfg(tests)]
// mod tests {

//     #[test]
//     fn main_code_test() {

//     }
// }
