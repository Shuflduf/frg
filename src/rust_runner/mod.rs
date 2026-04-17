use std::{
    env,
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

pub fn run(code: &str) -> Result<String, Box<dyn Error>> {
    let cache = env::var("XDG_CACHE_HOME").map_or_else(
        |_| {
            let home = env::var("HOME").expect("HOME not set");
            PathBuf::from(home).join(".cache")
        },
        PathBuf::from,
    );
    fs::create_dir_all(&cache)?;
    let file_path = cache.join("tmp_frg.rs");
    let binary_path = cache.join("tmp_frg");
    write_code_to_file(code, &file_path)?;

    match Command::new("rustc")
        .arg("--color=never")
        .arg("-o")
        .arg(&binary_path)
        .arg(&file_path)
        .output()
    {
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            return Err(
                "rustc not found. Please install Rust (https://rust-lang.org/tools/install/)"
                    .into(),
            );
        }
        Err(e) => {
            println!("{e}");
            return Err(e.into());
        }
        Ok(res) if !res.status.success() => {
            return Err(str::from_utf8(&res.stderr)?.into());
        }
        Ok(_) => {}
    }

    let execute_res = Command::new(binary_path)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    println!();

    Ok(String::from_utf8(execute_res.stdout)?)
}

fn write_code_to_file(code: &str, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let mut code_file = File::create(file_path).unwrap();
    let _ = code_file.write(code.as_bytes())?;
    Ok(())
}
