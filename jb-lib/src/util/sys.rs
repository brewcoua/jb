use std::path::PathBuf;
use anyhow::{bail, Context, Result};

pub fn mktemp_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("mktemp")
        .arg("-d")
        .output()
        .with_context(|| "Failed to create temporary directory")?;

    if !output.status.success() {
        bail!("Failed to create temporary directory: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
}

pub fn get_binary_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("systemd-path")
        .arg("user-binaries")
        .output()
        .with_context(|| "Failed to get user binary directory")?;

    if !output.status.success() {
        bail!("Failed to get user binary directory: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
}