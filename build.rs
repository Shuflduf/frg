fn main() {
    let package = "tree-sitter-froog";
    let source_directory = format!("ts-test-please/src");
    let source_file = format!("{source_directory}/parser.c");

    cc::Build::new()
        .file(&source_file)
        .include(&source_directory)
        .compile(&package);
}
