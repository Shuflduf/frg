use std::env;

pub const EXAMPLES: &[(&str, &str)] = &[
    ("fibonacci", include_str!("../examples/fibonacci.frg")),
    ("factorial", include_str!("../examples/factorial.frg")),
];

#[derive(Debug, Default)]
pub struct FrgArguments {
    pub help: bool,
    pub verbose: bool,
    pub dont_execute: bool,
    pub example: bool,
    pub file_name: Option<String>,
}
pub fn get_args() -> FrgArguments {
    let raw_args = &env::args().collect::<Vec<String>>()[1..];
    FrgArguments {
        help: raw_args.contains(&"-h".to_string()) || raw_args.contains(&"--help".to_string()),
        verbose: raw_args.contains(&"-v".to_string())
            || raw_args.contains(&"--verbose".to_string()),
        dont_execute: raw_args.contains(&"-n".to_string())
            || raw_args.contains(&"--no-exec".to_string()),
        example: raw_args.contains(&"-e".to_string())
            || raw_args.contains(&"--example".to_string()),
        file_name: raw_args
            .iter()
            .filter(|a| !a.starts_with('-'))
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>()
            .first()
            .cloned(),
    }

    // let mut i = 0;
    // while i < raw_args.len() {
    //     match raw_args[i].as_str() {
    //         "-h" | "--help" => args.help = true,
    //         "-v" | "--verbose" => args.verbose = true,
    //         "-n" | "--no-exec" => args.dont_execute = true,
    //         "-e" | "--example" => args.example = true,
    //         _ => {}
    //     }
    //     i += 1;
    // }

    // args
}

pub fn print_help() {
    let ver = env!("CARGO_PKG_VERSION");
    let lines = [
        format!(
            "frg Transpiler and Runner ({})\n",
            b(u("v".to_string() + ver))
        ),
        format!("{} {} [FILE]\n", bu("Usage:".into()), b("frg".into())),
        bu("Options:".into()).clone(),
        format!("  {:<23} Print this help menu.", b("-h, --help".into())),
        format!(
            "  {:<23} Show input code and intermediary treesitter, AST, and Rust.",
            b("-v, --verbose".into())
        ),
        format!(
            "  {:<23} Don't run generated Rust. Usually used with -v.",
            b("-n, --no-exec".into())
        ),
        format!(
            "  {:<23} Run an example. No args provided will list the examples.",
            b("-e, --example".into())
        ),
        String::new(),
        bu("Examples:".into()).clone(),
        format!("  {:<31} Execute a frg file.", b("frg my_code.frg".into())),
        format!("  {:<31} List examples", b("frg --example".into())),
        format!(
            "  {:<31} Run the fibonacci example",
            b("frg -e fibonacci".into())
        ),
        format!(
            "  {:<31} Show the process of a frg file being turned into Rust.",
            b("frg -v -n my_code.frg".into())
        ),
    ];
    let text = lines.join("\n");
    println!("{text}");
}

pub fn print_examples() {
    println!("{} (Run with frg -e [EXAMPLE])", bu("Examples:".into()));
    EXAMPLES.iter().for_each(|e| println!("  - {}", e.0));
}

// underline
pub fn u(text: String) -> String {
    format!("\x1b[4m{text}\x1b[24m")
}

// bold
pub fn b(text: String) -> String {
    format!("\x1b[1m{text}\x1b[22m")
}

// dim
pub fn d(text: String) -> String {
    format!("\x1b[2m{text}\x1b[22m")
}

pub fn bu(text: String) -> String {
    b(u(text))
}
