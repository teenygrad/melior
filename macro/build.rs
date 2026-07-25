use cargo_metadata::MetadataCommand;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str,
};

fn main() -> Result<(), Box<dyn Error>> {
    // When building with x.py, LLVM_CONFIG is set by bootstrap and points directly at the
    // build-dir llvm-config, which already has the mlir-c headers we need (LLVM is built with
    // the `mlir` project enabled per `compiler/rustc_llvm/llvm.toml`). Otherwise fall back to
    // cargo_metadata-based discovery for standalone (non-x.py) builds.
    let bin_dir: PathBuf = if let Ok(llvm_config) = std::env::var("LLVM_CONFIG") {
        PathBuf::from(llvm_config).parent().unwrap().to_path_buf()
    } else {
        let metadata = MetadataCommand::new().exec().unwrap();
        let target_dir: PathBuf = metadata.target_directory.into();
        target_dir.join("install/bin")
    };

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
