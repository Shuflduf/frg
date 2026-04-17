use crate::ast::{Expression, expressions};

fn rand_func(lowest: String, highest: String) -> String {
    stringify!({
        let min = LOWEST_PLACEHOLDER;
        let max = HIGHEST_PLACEHOLDER;
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let mut state = (duration.as_nanos() as u32) ^ 0x9e3779b9;
        state ^= state << 13;
        state ^= state >> 17;
        state ^= state << 5;

        min + (state as i32).abs() % (max - min + 1)
    })
    .replace("LOWEST_PLACEHOLDER", &lowest.to_string())
    .replace("HIGHEST_PLACEHOLDER", &highest.to_string())
}

pub fn transpile(params: &[Expression]) -> String {
    let insides: Vec<String> = params.iter().map(expressions::transpile).collect();
    rand_func(insides[0].clone(), insides[1].clone())
}
