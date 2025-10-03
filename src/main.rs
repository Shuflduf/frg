use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
        eprintln!("Usage: frg <code>.frg");
        return;
    }

    let file_path = &args[1];
}
