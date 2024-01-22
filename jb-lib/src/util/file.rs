use std::path::PathBuf;
use std::cmp::min;
use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

pub async fn download_file(url: &str, path: &PathBuf, size: u64) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")?
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {file_name}"));

    let mut file = File::create(path)?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
        let new = min(downloaded + (chunk.len() as u64), size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {file_name}"));
    Ok(())
}

pub fn extract_archive(path: &PathBuf, destination: &PathBuf, strip: u8) -> Result<(), Box<dyn std::error::Error>> {
    let output = std::process::Command::new("tar")
        .arg("--strip-components")
        .arg(strip.to_string())
        .arg("-xzf")
        .arg(path)
        .arg("-C")
        .arg(destination)
        .output()?;

    if !output.status.success() {
        return Err(format!("Failed to extract archive {}: {}", path.display(), String::from_utf8_lossy(&output.stderr)).as_str())?;
    }

    Ok(())
}