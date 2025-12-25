use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

pub fn run(code: &str) -> Result<(), Box<dyn Error>> {
    let cache_dir = env!("XDG_CACHE_HOME");
    let file_path = format!("{cache_dir}/tmp_frg.rs");
    println!("{file_path}");
    let mut code_file = File::create(file_path)?;
    code_file.write(code.as_bytes())?;

    Ok(())
}
