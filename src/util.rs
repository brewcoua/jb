//! Module for utilities.

use std::path::PathBuf;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

/// Download a file from a URL to a path.
///
/// # Errors
/// This function will return an error if the file cannot be downloaded, or if the file cannot be written to.
///
/// # Panics
/// This function will panic if the runtime cannot be started.
pub fn download(url: &str, path: &PathBuf, size: Option<u64>) -> anyhow::Result<()> {
    let span = tracing::debug_span!("download", url = url, path = path.display().to_string());
    let _guard = span.enter();

    tracing::debug!("start");

    tokio::task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let response = reqwest::get(url)
                    .await
                    .with_context(|| format!("Failed to download {url}"))?;

                let size = size.unwrap_or_else(|| response.content_length().unwrap_or(0));

                let mut file = tokio::fs::File::create(&path)
                    .await
                    .with_context(|| format!("Failed to create {}", path.display()))?;

                let mut downloaded_size: u64 = 0;
                let mut stream = response.bytes_stream();

                tracing::debug!("downloading chunks with size {size}");

                while let Some(chunk) = stream.next().await {
                    let chunk =
                        chunk.with_context(|| format!("Failed to download chunk at {url}"))?;
                    file.write_all(&chunk)
                        .await
                        .with_context(|| format!("Failed to write to {}", path.display()))?;
                    let new = std::cmp::min(downloaded_size + (chunk.len() as u64), size);
                    downloaded_size = new;
                }

                Ok::<(), anyhow::Error>(())
            })
    })?;

    tracing::debug!("done");

    Ok(())
}

/// Extract an archive to a destination.
///
/// # Errors
/// This function will return an error if the archive cannot be extracted.
pub fn extract_archive(path: &PathBuf, destination: &PathBuf, strip: u8) -> anyhow::Result<()> {
    let span = tracing::debug_span!("extract_archive", path = path.display().to_string(), destination = destination.display().to_string());
    let _guard = span.enter();

    tracing::debug!("start");

    let output = std::process::Command::new("tar")
        .arg("--strip-components")
        .arg(format!("{strip}"))
        .arg("-xf")
        .arg(path)
        .arg("-C")
        .arg(destination)
        .output()
        .with_context(|| format!("Failed to extract {}", path.display()))?;

    if !output.status.success() {
        anyhow::bail!("Failed to extract {}: {}", path.display(), String::from_utf8_lossy(&output.stderr));
    }

    tracing::debug!("done");

    Ok(())
}