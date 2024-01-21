use std::path::PathBuf;
use std::cmp::min;
use std::fs::File;
use std::io::Write;

use indicatif::{ProgressBar, ProgressStyle};
use futures_util::StreamExt;

pub fn download_file(url: &str, path: &PathBuf, size: u64) -> Result<(), Box<dyn std::error::Error>> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()?;
    rt.block_on(download_file_stream(url, path, size))
}

pub async fn download_file_stream(url: &str, path: &PathBuf, size: u64) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await.or_else(|_| {
        log::error!("Failed to download file from {}", url);
        Err("Failed to download file")
    })?;

    let file_name = path.file_name().unwrap().to_str().unwrap();

    let pb = ProgressBar::new(size);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .expect("Failed to set progress bar style")
        .progress_chars("#>-"));
    pb.set_message(format!("Downloading {file_name}"));

    let mut file = File::create(path).or_else(|_| {
        log::error!("Failed to create file {}", path.display());
        Err("Failed to create file")
    })?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.or_else(|_| {
            log::error!("Failed to read chunk from response");
            Err("Failed to read chunk")
        })?;
        file.write_all(&chunk).or_else(|_| {
            log::error!("Failed to write chunk to file {}", path.display());
            Err("Failed to write chunk")
        })?;
        let new = min(downloaded + (chunk.len() as u64), size);
        downloaded = new;
        pb.set_position(new);
    }

    pb.finish_with_message(format!("Downloaded {file_name}"));
    Ok(())
}

pub fn extract_archive(path: &PathBuf, destination: &PathBuf, strip: u8) -> Result<(), &'static str> {
    let output = std::process::Command::new("tar")
        .arg("--strip-components")
        .arg(strip.to_string())
        .arg("-xzf")
        .arg(path)
        .arg("-C")
        .arg(destination)
        .output()
        .or_else(|_| {
            log::error!("Failed to extract archive {}", path.display());
            Err("Failed to extract archive")
        })?;

    if !output.status.success() {
        log::error!("Failed to extract archive {}", path.display());
        log::error!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to extract archive")?;
    }

    Ok(())
}