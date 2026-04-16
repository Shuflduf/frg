mod tmp;

fn main() {
    let language = "frg";
    let package = format!("tree-sitter-{language}");
    let source_directory = format!("{package}/src");
    let source_file = format!("{source_directory}/parser.c");

    println!("cargo:rerun-if-changed={source_file}");

    cc::Build::new()
        .std("c11")
        .file(source_file)
        .include(source_directory)
        .compile(&package);
}
