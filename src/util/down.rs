use std::cmp::min;
use std::io::{self, Read};
use std::path::PathBuf;

use anyhow::Context;
use flate2::read::GzDecoder;
use futures_lite::StreamExt;
use tar::Archive;

pub fn download_extract(url: &str, folder: &PathBuf, progress: Option<&indicatif::ProgressBar>) -> anyhow::Result<()> {
    let filename = url.split('/').last().expect("Failed to get filename");
    if !filename.ends_with(".tar.gz") {
        anyhow::bail!("Invalid file type: {}", filename);
    }

    if !folder.exists() {
        std::fs::create_dir_all(&folder)
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

                    // Move all content of the unpacked folder into the parent folder (the specified folder)
                    let mut entries = std::fs::read_dir(&folder)
                        .with_context(|| format!("Failed to read {}", folder.display()))?;
                    while let Some(entry) = entries.next() {
                        let entry = entry
                            .with_context(|| format!("Failed to read {}", folder.display()))?;
                        let path = entry.path();
                        let new_path = folder.join(path.file_name().unwrap());
                        std::fs::rename(&path, &new_path)
                            .with_context(|| format!("Failed to move {} to {}", path.display(), new_path.display()))?;
                    }

                    std::fs::remove_dir(&folder)
                        .with_context(|| format!("Failed to remove {}", folder.display()))?;

                    Ok::<(), anyhow::Error>(())
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

                        tx.send_async(chunk.to_vec()).await
                            .with_context(|| "Failed to send chunk")?;
                    }
                    drop(tx);
                } else {
                    anyhow::bail!("Failed to fetch {url}: {}", response.status());
                }

                tokio::task::spawn_blocking(|| decoder_thread.join())
                    .await
                    .with_context(|| "Failed to join decoder thread")?
                    .map_err(|e| anyhow::anyhow!("Decoder thread panicked: {:?}", e))??;

                Ok(())
            })
    })
}

/// A read-only channel, used for decoding a tarball.
struct ChannelRead {
    rx: flume::Receiver<Vec<u8>>,
    current: io::Cursor<Vec<u8>>,
}

impl ChannelRead {
    fn new(rx: flume::Receiver<Vec<u8>>) -> Self {
        Self {
            rx,
            current: io::Cursor::new(vec![]),
        }
    }
}

impl Read for ChannelRead {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.current.position() as usize == self.current.get_ref().len() {
            if let Ok(data) = self.rx.recv() {
                self.current = io::Cursor::new(data);
            }
        }

        self.current.read(buf)
    }
}