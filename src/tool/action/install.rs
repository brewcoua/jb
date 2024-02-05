use std::path::PathBuf;
use anyhow::Context;
use tokio::io::AsyncWriteExt;
use futures_util::StreamExt;

use super::list::List;
use super::link::Link;
use crate::api;
use crate::tool::Tool;

pub trait Install {
    /// Returns whether the tool is installed.
    ///
    /// This will check if the tool's directory exists and if the tool is linked.
    ///
    /// # Errors
    /// This function will return an error if the tool's directory cannot be checked.
    fn is_installed(&self) -> anyhow::Result<bool> where Self: Sized;

    /// Installs the tool.
    ///
    /// This will download the tool's release, extract it, and link the tool.
    ///
    /// # Errors
    /// This function will return an error if the tool is already installed, or if the tool cannot be installed.
    fn install(&mut self) -> anyhow::Result<()> where Self: Sized;

    /// Uninstalls the tool.
    ///
    /// This will remove the tool's directory and unlink the tool.
    ///
    /// # Errors
    /// This function will return an error if the tool is not installed, or if the tool cannot be removed.
    fn uninstall(&self) -> anyhow::Result<()> where Self: Sized;
}

impl Install for Tool {
    fn is_installed(&self) -> anyhow::Result<bool> {
        let span = tracing::debug_span!("is_installed", tool = self.as_str());
        let _enter = span.enter();

        tracing::debug!("Checking if installed");

        Ok(
            Tool::list_kind(self.kind)
                .with_context(|| format!("Failed to list installed tools for {}", self.kind))?
                .iter()
                .any(|tool| self.matched(tool))
        )
    }

    fn install(&mut self) -> anyhow::Result<()> {
        let span = tracing::info_span!("install", tool = self.as_str());
        let _enter = span.enter();

        let result = api::fetch::release(self)?;

        tracing::info!("Found release: {}", result.tool);

        self.version = Some(result.tool.version.unwrap());
        self.build = Some(result.tool.build.unwrap());
        self.release = Some(result.tool.release.unwrap());

        let installed = self.is_installed()?;
        if installed {
            tracing::warn!("{} is already installed", self.as_str());
            return Ok(());
        }

        let tempdir = tempfile::tempdir()
            .with_context(|| "Failed to create temporary directory")?;

        let process = || {
            let archive_name = result
                .download.link
                .rsplit('/')
                .next()
                .with_context(|| "Failed to get archive name")?;

            tracing::info!("Downloading {} ({})", archive_name, humansize::format_size(result.download.size, humansize::DECIMAL));

            let archive_path = tempdir.path().join(archive_name);
            download(&result.download.link, &archive_path, Some(result.download.size))?;

            let tool_directory = self.as_path();
            if !tool_directory.exists() {
                std::fs::create_dir_all(&tool_directory)
                    .with_context(|| format!("Failed to create {}", tool_directory.display()))?;
            }

            tracing::info!("Extracting to {}", tool_directory.display());

            extract_archive(&archive_path, &tool_directory, 1)?;

            tracing::info!("Linking {}", self.as_str());
            self.link()?;

            Ok(())
        };

        let output = process();

        // Clean up the temporary directory
        tempdir.close()
            .with_context(|| "Failed to clean up temporary directory")?;

        match output {
            Ok(()) => {
                tracing::info!("Installed {}", self.as_str());
                Ok(())
            }
            Err(e) => {
                tracing::error!("Failed to install {}: {e}", self.as_str());
                Err(e)
            }
        }
    }

    fn uninstall(&self) -> anyhow::Result<()> {
        let span = tracing::info_span!("uninstall", tool = self.as_str());
        let _enter = span.enter();

        let installed = self.is_installed()?;
        if !installed {
            tracing::warn!("{} is not installed", self.as_str());
            return Ok(());
        }

        tracing::info!("Unlinking {}", self.as_str());
        self.unlink()?;

        let tool_directory = self.as_path();
        tracing::info!("Removing {}", tool_directory.display());
        std::fs::remove_dir_all(&tool_directory)
            .with_context(|| format!("Failed to remove {}", tool_directory.display()))?;

        tracing::info!("Uninstalled {}", self.as_str());

        Ok(())
    }
}

fn download(url: &str, path: &PathBuf, size: Option<u64>) -> anyhow::Result<()> {
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

fn extract_archive(path: &PathBuf, destination: &PathBuf, strip: u8) -> anyhow::Result<()> {
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