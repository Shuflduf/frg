use crate::ast::{Expression, expressions};

fn input_func(prompt: String) -> String {
    let code = stringify!({
        PROMPT_PLACEHOLDER;
        use std::io::Write;
        std::io::stdout().flush().unwrap();
        let mut tmp_str_frg = String::new();
        std::io::stdin()
            .read_line(&mut tmp_str_frg)
            .expect("Failed to read line");
        tmp_str_frg = tmp_str_frg.trim().into();
        tmp_str_frg.leak()
    })
    .replace("PROMPT_PLACEHOLDER", &format!("print!({prompt})"));
    code.into()
}

pub fn transpile(params: &[Expression]) -> String {
    let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    // format!("println!({insides})")
    // let insides = expressions::transpile_list(&params.iter().map(expressions::transpile).collect());
    format!("{}", input_func(insides))
}
