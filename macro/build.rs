use cargo_metadata::MetadataCommand;
use std::{
    error::Error,
    path::{Path, PathBuf},
    process::Command,
    str,
};

fn main() -> Result<(), Box<dyn Error>> {
    // When building with x.py, LLVM_CONFIG is set by bootstrap and points to the build-dir
    // llvm-config. For melior we need the install dir (which has mlir-c headers).
    // Derive it from CARGO_MANIFEST_DIR (src/melior/macro -> workspace_root -> target/install).
    // Otherwise fall back to cargo_metadata-based discovery.
    let bin_dir: PathBuf = if std::env::var("LLVM_CONFIG").is_ok() {
        let manifest_dir = PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap());
        let workspace_root =
            manifest_dir.parent().unwrap().parent().unwrap().parent().unwrap();
        workspace_root.join("target/install/bin")
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
