//! Module for utilities.

use std::cmp::min;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Context;
use flate2::read::GzDecoder;
use futures_lite::StreamExt;
use notify_rust::{Hint, Notification, Timeout};
use tar::Archive;
use sha2::{Digest, Sha256};

/// Download and extract a tarball from a URL.
///
/// It will directly stream the download to the extraction, so it will not store the tarball on disk.
///
/// # Errors
/// This function will return an error if the download or extraction fails.
///
/// # Panics
/// This function will panic if the folder cannot be created or written to.
pub fn download_extract(
    url: &str,
    folder: &PathBuf,
    checksum_url: Option<&str>,
    progress: Option<&indicatif::ProgressBar>,
) -> anyhow::Result<()> {
    let filename = url.split('/').last().expect("Failed to get filename");
    if !filename.ends_with(".tar.gz") {
        anyhow::bail!("Invalid file type: {}", filename);
    }

    if !folder.exists() {
        std::fs::create_dir_all(folder)
            .with_context(|| format!("Failed to create {}", folder.display()))?;
    }

    tokio::task::block_in_place(|| {
        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let client = reqwest::Client::new();

                let response = client.get(url)
                    .send()
                    .await
                    .with_context(|| format!("Failed to fetch {url}"))?;

                let (tx, rx) = flume::bounded(0);

                let folder = folder.clone();
                let decoder_thread = std::thread::spawn(move || {
                    let input = ChannelRead::new(rx);
                    let gz = GzDecoder::new(input);
                    let mut archive = Archive::new(gz);

                    archive.unpack(&folder)
                        .with_context(|| format!("Failed to extract to {}", folder.display()))?;

                    strip_content(&folder)
                        .with_context(|| format!("Failed to strip content of {}", folder.display()))?;

                    let channel = archive.into_inner().into_inner();
                    Ok::<String, anyhow::Error>(channel.hash())
                });

                if response.status() == reqwest::StatusCode::OK {
                    let content_length = response.content_length().unwrap_or(0);
                    let mut downloaded = 0;

                    let mut stream = response.bytes_stream();
                    while let Some(chunk) = stream.next().await {
                        let chunk = chunk
                            .with_context(|| "Failed to fetch chunk")?;

                        downloaded += chunk.len() as u64;
                        if let Some(pb) = &progress {
                            pb.set_length(content_length);
                            pb.set_position(min(downloaded, content_length));
                        }

                        if let Err(err) = tx.send(chunk.to_vec()) {
                            match err.to_string().as_str() {
                                "sending on a closed channel" => break,
                                _ => anyhow::bail!("Failed to send chunk: {}", err),
                            }
                        }
                    }
                    drop(tx);
                } else {
                    anyhow::bail!("Failed to fetch {url}: {}", response.status());
                }

                let hash = tokio::task::spawn_blocking(|| decoder_thread.join())
                    .await
                    .with_context(|| "Failed to join decoder thread")?
                    .map_err(|e| anyhow::anyhow!("Decoder thread panicked: {:?}", e))??;

                // If we have a checksum URL, we should check the hash
                if let Some(checksum_url) = checksum_url {
                    let checksum = client.get(checksum_url)
                        .send()
                        .await
                        .with_context(|| format!("Failed to fetch {checksum_url}"))?
                        .text()
                        .await
                        .with_context(|| format!("Failed to read {checksum_url}"))?;

                    let checksum = checksum.split_whitespace().next().unwrap();
                    if checksum.trim() != hash {
                        anyhow::bail!("Checksum mismatch: expected {checksum}, got {hash}");
                    }
                }

                Ok(())
            })
    })
}

/// A read-only channel, used for decoding a tarball.
struct ChannelRead {
    rx: flume::Receiver<Vec<u8>>,
    current: io::Cursor<Vec<u8>>,
    hash: Sha256,
}

impl ChannelRead {
    fn new(rx: flume::Receiver<Vec<u8>>) -> Self {
        Self {
            rx,
            current: io::Cursor::new(vec![]),
            hash: Sha256::new(),
        }
    }

    pub fn hash(self) -> String {
        format!("{:x}", self.hash.finalize())
    }
}

impl Read for ChannelRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.current.position() == self.current.get_ref().len() as u64 {
            if let Ok(data) = self.rx.recv() {
                self.hash.update(&data);
                self.current = io::Cursor::new(data);
            }
        }

        self.current.read(buf)
    }
}

/// Strip the content of a folder, moving all files and folders one level up.
///
/// # Errors
/// This function will return an error if any file or folder cannot be moved.
///
/// # Panics
/// This function will panic if the folder cannot be read or written to.
pub fn strip_content(folder: &PathBuf) -> anyhow::Result<()> {
    let entries = std::fs::read_dir(folder)
        .with_context(|| format!("Failed to read {}", folder.display()))?;
    for entry in entries {
        let entry = entry
            .with_context(|| format!("Failed to read {}", folder.display()))?;

        if !entry.file_type().unwrap().is_dir() {
            let path = entry.path();
            let new_path = folder.join(path.file_name().unwrap());
            std::fs::rename(&path, &new_path)
                .with_context(|| format!("Failed to move {} to {}", path.display(), new_path.display()))?;
            continue;
        }

        let path = entry.path();
        let entries = std::fs::read_dir(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;

        for entry in entries {
            let entry = entry
                .with_context(|| format!("Failed to read {}", path.display()))?;
            let path = entry.path();
            let new_path = folder.join(path.file_name().unwrap());
            std::fs::rename(&path, &new_path)
                .with_context(|| format!("Failed to move {} to {}", path.display(), new_path.display()))?;
        }

        std::fs::remove_dir(&path)
            .with_context(|| format!("Failed to remove {}", path.display()))?;
    }
    Ok(())
}

/// Show a desktop notification.
///
/// # Errors
/// This function will return an error if the notification cannot be shown.
pub fn notify(msg: &str, icon: &str) -> anyhow::Result<()> {
    let result = Notification::new()
        .summary("JetBrains CLI")
        .body(msg)
        .icon(icon)
        .appname("jb")
        .hint(Hint::Category("Development".to_owned()))
        .hint(Hint::Category("IDE".to_owned()))
        .timeout(Timeout::Milliseconds(4000))
        .show();

    if let Err(e) = result {
        anyhow::bail!("Failed to show notification: {}", e);
    }
    Ok(())
}
