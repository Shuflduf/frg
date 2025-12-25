use std::{error::Error, fs::File, io::Write, process::Command};

pub fn run(code: &str) -> Result<(), Box<dyn Error>> {
    let cache_dir = env!("XDG_CACHE_HOME");
    let file_path = format!("{cache_dir}/tmp_frg.rs");
    let binary_path = format!("{cache_dir}/tmp_frg");
    write_code_to_file(code, &file_path)?;

    let creation_res = Command::new("rustc")
        .arg("-o")
        .arg(&binary_path)
        .arg(&file_path)
        .output()?;
    if !creation_res.status.success() {
        return Err(format!(
            "Failed to compile program: {:?}",
            str::from_utf8(&creation_res.stderr)?
        )
        .into());
    }

    let exectute_res = Command::new(binary_path).output()?;
    println!("=== Frg Result ===");
    println!("{}", str::from_utf8(&exectute_res.stdout)?);
    println!("==================");

    Ok(())
}

fn write_code_to_file(code: &str, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut code_file = File::create(file_path)?;
    let _ = code_file.write(code.as_bytes())?;
    Ok(())
}
