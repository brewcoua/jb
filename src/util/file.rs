use std::cmp::min;
use std::path::{Path, PathBuf};

use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::task::block_in_place;

use anyhow::{bail, Context, Result};
use futures_util::StreamExt;

/// Download a file from a URL to a path.
///
/// This function will download the file at the given URL to the given path, using a stream as to avoid loading the entire file into memory.
///
/// # Errors
/// This function will return an error if the file could not be downloaded (e.g. the URL is invalid).
/// This function will also return an error if the file could not be created or written to.
pub fn download(url: &str, path: &PathBuf, size: Option<u64>) -> Result<()> {
    let span = tracing::info_span!("download", url = url, path = path.display().to_string());
    let _guard = span.enter();

    tracing::info!("Downloading");

    block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let response = reqwest::get(url)
                    .await
                    .with_context(|| format!("Failed to download {url}"))?;

                let size = size.unwrap_or_else(|| response.content_length().unwrap_or(0));

                let mut file = File::create(path)
                    .await
                    .with_context(|| format!("Failed to create {}", path.display()))?;

                let mut downloaded_size: u64 = 0;
                let mut stream = response.bytes_stream();

                while let Some(chunk) = stream.next().await {
                    let chunk =
                        chunk.with_context(|| format!("Failed to download chunk at {url}"))?;
                    file.write_all(&chunk)
                        .await
                        .with_context(|| format!("Failed to write to {}", path.display()))?;
                    let new = min(downloaded_size + (chunk.len() as u64), size);
                    downloaded_size = new;
                }

                Ok::<(), anyhow::Error>(())
            })
    })?;

    tracing::info!("Downloaded");

    Ok(())
}

/// Extract an archive to a destination.
///
/// This function will extract the archive at the given path to the given destination.
/// It uses the `tar` command to extract the archive so it may not work on Windows.
///
/// # Errors
/// This function will return an error if:
/// - the archive could not be extracted.
/// - the `tar` command could not be found.
/// - the `tar` command failed.
pub fn extract_archive(path: &PathBuf, destination: &PathBuf, strip: u8) -> Result<()> {
    let span = tracing::info_span!("extract_archive", path = path.display().to_string(), destination = destination.display().to_string());
    let _guard = span.enter();

    tracing::info!("Extracting");

    let output = std::process::Command::new("tar")
        .arg("--strip-components")
        .arg(strip.to_string())
        .arg("-xzf")
        .arg(path)
        .arg("-C")
        .arg(destination)
        .output()?;

    if !output.status.success() {
        tracing::error!("Failed to extract archive {}", path.display());
        bail!("Failed to extract archive {}", path.display());
    }

    tracing::info!("Extracted");

    Ok(())
}

/// Check the integrity of a file using a checksum file.
///
/// This function will check the integrity of the given file using the given checksum file.
///
/// # Errors
/// This function will return an error if:
/// - the checksum could not be checked.
/// - the `sha256sum` command could not be found.
/// - the `sha256sum` command failed.
/// - the checksum did not match.
pub fn checksum(checksum: &Path, file: &Path) -> Result<()> {
    let span = tracing::debug_span!("checksum", checksum = checksum.display().to_string(), file = file.display().to_string());
    let _guard = span.enter();

    tracing::debug!("Checking checksum");

    let cwd = checksum.parent().unwrap();

    let output = std::process::Command::new("sha256sum")
        .arg("-c")
        .arg(checksum.file_name().unwrap())
        .current_dir(cwd)
        .output()?;

    if !output.status.success() {
        tracing::error!("Checksum failed for {}:\n{}", file.display(), String::from_utf8_lossy(&output.stderr));
        bail!("Checksum failed for {}:\n{}", file.display(), String::from_utf8_lossy(&output.stderr));
    }

    tracing::debug!("Checksum passed");

    Ok(())
}