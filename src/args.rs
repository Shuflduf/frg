use std::env;

#[derive(Debug)]
pub struct FrgArguments {
    pub help: bool,
    pub verbose: bool,
    pub dont_execute: bool,
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
        file_name: raw_args
            .iter()
            .filter(|a| !a.starts_with('-'))
            .map(std::borrow::ToOwned::to_owned)
            .collect::<Vec<String>>()
            .first()
            .cloned(),
    }
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
        format!("  {:<23} Print help", b("-h, --help".into())),
        format!(
            "  {:<23} Show intermediary treesitter, AST, and Rust",
            b("-v, --verbose".into())
        ),
        format!(
            "  {:<23} Don't run generated Rust",
            b("-n, --no-exec".into())
        ),
    ];
    let text = lines.join("\n");
    println!("{text}");
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
