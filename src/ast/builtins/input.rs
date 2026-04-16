use crate::ast::{Expression, expressions};

fn input_func(prompt: String) -> String {
    stringify!({
        use std::io::Write;
        PROMPT_PLACEHOLDER;
        std::io::stdout().flush().unwrap();
        let mut tmp_str_frg = String::new();
        std::io::stdin()
            .read_line(&mut tmp_str_frg)
            .expect("Failed to read line");
        tmp_str_frg = tmp_str_frg.trim().into();
        tmp_str_frg.leak()
    })
    .replace("PROMPT_PLACEHOLDER", &format!("print!({prompt})"))
}

pub fn transpile(params: &[Expression]) -> String {
    let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    // format!("println!({insides})")
    // let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    input_func(insides).clone()
}
