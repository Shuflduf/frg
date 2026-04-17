use std::fs;
use tree_sitter::{Language, Parser};

use crate::args::{FrgArguments, bu, d};

mod args;
pub mod ast;
mod rust_runner;
mod rust_transpiler;

unsafe extern "C" {
    fn tree_sitter_frg() -> Language;
}

fn main() {
    let frg_args = args::get_args();
    if frg_args.help {
        args::print_help();
        return;
    }

    let input = match get_input_text(&frg_args) {
        Some(val) => val,
        None => return,
    };

    println!();

    let language = unsafe { tree_sitter_frg() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let treesitter_tree = parser.parse(&input, None).unwrap();
    if frg_args.verbose {
        println!(
            "{}\n{}\n{}\n",
            header_text("Treesitter", true),
            body_text(&treesitter_tree.root_node().to_string()),
            header_text("Treesitter", false)
        );
    }
    let ast_tree = ast::build(&treesitter_tree, &input);
    let rust_code = rust_transpiler::transpile(&ast_tree);
    // println!("\n\n{rust_code}\n\n");
    if frg_args.verbose {
        section("AST", &format!("{ast_tree:?}"));
        section("frg Code", &input);
        section("Rust Code", &rust_code);
    }
    if !frg_args.dont_execute {
        println!("{}", header_text("Output", true));
        match rust_runner::run(&rust_code) {
            Ok(output) => println!("{}{}", &output, header_text("Output", false)),
            Err(e) => section("rustc Error", &e.to_string()),
        }
    }
}

fn get_input_text(args: &FrgArguments) -> Option<String> {
    if args.file_name.is_none() {
        if args.example {
            args::print_examples();
        } else {
            args::print_help();
        }
        return None;
    }
    let content = if args.example {
        args::EXAMPLES
            .iter()
            .find(|e| e.0 == args.file_name.as_ref().unwrap())
            .expect("Example not found")
            .1
            .to_string()
    } else {
        fs::read_to_string(args.file_name.as_ref().unwrap()).expect("File not found")
    };
    Some(content)
}

fn section(header: &str, text: &str) {
    println!(
        "{}\n{}\n{}\n",
        header_text(header, true),
        body_text(text),
        header_text(header, false)
    );
}

fn body_text(text: &str) -> String {
    d(text.into())
}

fn header_text(text: &str, opening: bool) -> String {
    if opening {
        bu(format!(">>> {text} >>>"))
    } else {
        bu(format!("<<< {text} <<<"))
    }
}
