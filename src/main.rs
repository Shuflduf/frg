use std::fs;

mod ast;
mod ast_nodes;
mod lexer;

fn main() {
    // let args = env::args().collect::<Vec<String>>();
    // if args.len() < 2 {
    //     eprintln!("Usage: frg <code>.frg");
    //     return;
    // }

    // let file_path = &args[1];
    // let input = fs::read_to_string("code.frog").unwrap();
    let input = "bool result = false int thing = 5";
    let tokens = lexer::lex(input);
    let tree = ast::parse(tokens);
    println!("{tree:?}");
}

// #[cfg(tests)]
// mod tests {

//     #[test]
//     fn main_code_test() {

//     }
// }
