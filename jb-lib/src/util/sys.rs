use anyhow::{bail, Context, Result};
use std::path::PathBuf;

/// Create a temporary directory using `mktemp`.
///
/// This function will create a temporary directory using `mktemp` and return the path to it.
///
/// # Errors
/// This function will return an error if `mktemp` fails to create a temporary directory.
pub fn mktemp_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("mktemp")
        .arg("-d")
        .output()
        .with_context(|| "Failed to create temporary directory")?;

    if !output.status.success() {
        bail!(
            "Failed to create temporary directory: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}

/// Get the user binary directory using `systemd-path`.
///
/// This function will get the user binary directory using `systemd-path` and return the path to it.
///
/// # Errors
/// This function will return an error if `systemd-path` fails to get the user binary directory.
pub fn get_binary_dir() -> Result<PathBuf> {
    let output = std::process::Command::new("systemd-path")
        .arg("user-binaries")
        .output()
        .with_context(|| "Failed to get user binary directory")?;

    if !output.status.success() {
        bail!(
            "Failed to get user binary directory: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(PathBuf::from(
        String::from_utf8_lossy(&output.stdout).trim(),
    ))
}
