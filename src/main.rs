use std::{env, fs};
use tree_sitter::{Language, Parser};

mod ast_builder;

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

    let treesitter_tree = parser.parse(&input, None).unwrap();
    println!("{}", treesitter_tree.root_node().to_string());
    let ast = ast_builder::build(&treesitter_tree, &input);
    println!("{ast:?}")
}
