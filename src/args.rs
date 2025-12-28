use std::env;

#[derive(Debug)]
pub struct FrgArguments {
    pub verbose: bool,
    pub dont_execute: bool,
    pub file_name: Option<String>,
}
pub fn get_args() -> FrgArguments {
    let raw_args = &env::args().collect::<Vec<String>>()[1..];
    FrgArguments {
        verbose: raw_args.contains(&"-v".to_string()),
        dont_execute: raw_args.contains(&"-n".to_string()),
        file_name: raw_args
            .iter()
            .filter(|a| !a.starts_with("-"))
            .map(|a| a.to_owned())
            .collect::<Vec<String>>().first()
            .cloned(),
    }
}
