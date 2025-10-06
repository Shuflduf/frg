use std::fs;

mod lexer;

fn main() {
    // let args = env::args().collect::<Vec<String>>();
    // if args.len() < 2 {
    //     eprintln!("Usage: frg <code>.frg");
    //     return;
    // }

    // let file_path = &args[1];
    let input = fs::read_to_string("code.frog").unwrap();
    let tokens = lexer::Lexer::new(&input).parse();
    println!("{tokens:?}");
}

// #[cfg(tests)]
// mod tests {

//     #[test]
//     fn main_code_test() {

//     }
// }
