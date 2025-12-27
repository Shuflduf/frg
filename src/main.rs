use std::{env, fs};
use tree_sitter::{Language, Parser};

pub mod ast;
mod rust_runner;
mod rust_transpiler;

unsafe extern "C" {
    fn tree_sitter_frg() -> Language;
}

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let file_path = &args.get(1).map_or("examples/test.frg", |v| v);
    let input = fs::read_to_string(file_path).unwrap();
    println!("{input}");

    let language = unsafe { tree_sitter_frg() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let treesitter_tree = parser.parse(&input, None).unwrap();
    println!("{}", treesitter_tree.root_node());
    let ast_tree = ast::build(&treesitter_tree, &input);
    println!("{ast_tree:?}");
    let rust_code = rust_transpiler::transpile(&ast_tree);
    println!("{rust_code}\n");
    let _ = rust_runner::run(&rust_code);

    // (5, "str").
    // frog_ages.iter().
    // let res = rust_runner::run(&rust_code);
    // println!("{run_res:?}");
}
