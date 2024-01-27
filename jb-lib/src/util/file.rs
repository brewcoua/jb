use std::cmp::min;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};

/// Download a file from a URL to a path.
///
/// This function will download the file at the given URL to the given path.
/// It will also display a progress bar while downloading.
///
/// It will run asynchronously, so it needs to be awaited.
///
/// # Errors
/// This function will return an error if the file could not be downloaded (e.g. the URL is invalid).
/// This function will also return an error if the file could not be created or written to.
pub async fn download(url: &str, path: &PathBuf, size: u64) -> Result<()> {
    let response = reqwest::get(url)
        .await
        .with_context(|| format!("Failed to download {url}"))?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {file_name}"));

    let mut file =
        File::create(path).with_context(|| format!("Failed to create {}", path.display()))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.with_context(|| format!("Failed to download chunk at {url}"))?;
        file.write_all(&chunk)
            .with_context(|| format!("Failed to write to {}", path.display()))?;
        let new = min(downloaded + (chunk.len() as u64), size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {file_name}"));
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
    let output = std::process::Command::new("tar")
        .arg("--strip-components")
        .arg(strip.to_string())
        .arg("-xzf")
        .arg(path)
        .arg("-C")
        .arg(destination)
        .output()?;

    if !output.status.success() {
        bail!("Failed to extract archive {}", path.display());
    }

    Ok(())
}
