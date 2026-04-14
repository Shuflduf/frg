use std::fs;
use tree_sitter::{Language, Parser};

mod args;
pub mod ast;
mod rust_runner;
mod rust_transpiler;

unsafe extern "C" {
    fn tree_sitter_frg() -> Language;
}

fn main() {
    let frg_args = args::get_args();
    let input = fs::read_to_string(
        frg_args
            .file_name
            .unwrap_or("examples/test.frg".to_string()),
    )
    .unwrap();

    let language = unsafe { tree_sitter_frg() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let treesitter_tree = parser.parse(&input, None).unwrap();
    if frg_args.verbose {
        println!(
            ">>> Treesitter >>>\n{}\n<<< Treesitter <<<\n",
            treesitter_tree.root_node()
        );
    }
    let ast_tree = ast::build(&treesitter_tree, &input);
    let rust_code = rust_transpiler::transpile(&ast_tree);
    if frg_args.verbose {
        println!(">>> AST >>>\n{ast_tree:?}\n<<< AST <<<\n");
        println!(">>> frg Code >>>\n{input}\n<<< frg Code <<<\n");
        println!(">>> Rust Code >>>\n{rust_code}\n<<< Rust Code <<<\n");
    }
    if !frg_args.dont_execute {
        let _ = rust_runner::run(&rust_code)
            .inspect_err(|e| println!(">>> rustc error >>>\n{e}\n<<< rustc error <<<"));
    }
}
