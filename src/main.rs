use tree_sitter::{Language, Parser};

unsafe extern "C" {
    fn tree_sitter_froog() -> Language;
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_parser() {
    let language = unsafe { tree_sitter_froog() };
    let mut parser = Parser::new();
    parser.set_language(&language).unwrap();

    let source_code = "int val = 5 * 3";
    let tree = parser.parse(source_code, None).unwrap();

    assert_eq!(tree.root_node().to_sexp(), "(source_file)");
}
