fn main() {
    let mut num: &'static str = {
        print!("HIIIIIIIIIIIIII: ");
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        let mut tmp_str_frg = String::new();
        std::io::stdin()
            .read_line(&mut tmp_str_frg)
            .expect("Failed to read line");
        tmp_str_frg = tmp_str_frg.trim().into();
        tmp_str_frg.leak()
    };
    _ = println!("{num}");
}
