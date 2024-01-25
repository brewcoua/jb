use std::path::PathBuf;
use anyhow::{bail, Result};

pub fn mktemp_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("mktemp")
        .arg("-d")
        .output()
        .or_else(|err| Err(format!("Failed to create temporary directory: {err}")))?;

    if !output.status.success() {
        bail!("Failed to create temporary directory: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
}

pub fn get_binary_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("systemd-path")
        .arg("user-binaries")
        .output()
        .or_else(|err| Err(format!("Failed to get user binary directory: {err}")))?;

    if !output.status.success() {
        bail!("Failed to get user binary directory: {}", String::from_utf8_lossy(&output.stderr));
    }

    Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
}