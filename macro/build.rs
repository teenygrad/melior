use cargo_metadata::MetadataCommand;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str,
};

fn main() -> Result<(), Box<dyn Error>> {
    let metadata = MetadataCommand::new().exec().unwrap();
    let target_dir: PathBuf = metadata.target_directory.into();
    let bin_dir: PathBuf = target_dir.join("install/bin");

    println!(
        "cargo:rustc-env=LLVM_INCLUDE_DIRECTORY={}",
        // spell-checker: disable-next-line
        llvm_config(bin_dir.as_path(), "--includedir")?
    );

    Ok(())
}

fn llvm_config(bin_dir: &Path, argument: &str) -> Result<String, Box<dyn std::error::Error>> {
    let call = format!(
        "{} --link-static {}",
        bin_dir.join("llvm-config").display(),
        argument
    );

    Ok(str::from_utf8(
        &if cfg!(target_os = "windows") {
            Command::new("cmd").args(["/C", &call]).output()?
        } else {
            Command::new("sh").arg("-c").arg(&call).output()?
        }
        .stdout,
    )?
    .trim()
    .to_string())
}
