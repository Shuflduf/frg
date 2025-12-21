use std::{env, fs};

use tree_sitter::{Language, Parser};

unsafe extern "C" {
    fn tree_sitter_frg() -> Language;
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let file_path = &args.get(1).map_or("examples/test.frg", |v| v);
    let input = fs::read_to_string(file_path).unwrap();

    let language = unsafe { tree_sitter_frg() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let tree = parser.parse(input, None).unwrap();
    println!("{}", tree.root_node())
}
