//! Module for updating the CLI.

use std::path::PathBuf;
use anyhow::Context;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Release {
    tag_name: String,
    body: String,
}

impl Release {
    /// Get the latest release.
    pub fn latest() -> anyhow::Result<Self> {
        let repo = env!("CARGO_PKG_REPOSITORY");

        let url = repo.replace("github.com", "api.github.com/repos")
            + "/releases/latest";

        let client = reqwest::blocking::Client::new();

        let response = client.get(&url)
            .header("User-Agent", "JetBrains CLI")
            .send()
            .with_context(|| format!("Failed to get latest release from {url}"))?;

        let release = response.json::<Release>()
            .with_context(|| format!("Failed to parse latest release from {url}"))?;

        Ok(release)
    }

    /// Try updating the CLI, if there is a new release.
    pub fn try_update(location: &PathBuf, force: bool) -> anyhow::Result<(bool, String)> {
        let latest = Self::latest()?;
        let current = env!("CARGO_PKG_VERSION");
        let target = env!("TARGET");

        if latest.tag_name == format!("v{}", current) {
            if force {
                tracing::warn!("The latest version is already installed ({})", current);
                tracing::warn!("Forcing the update...");
            } else {
                tracing::error!("The latest version is already installed ({})", current);
                return Ok((false, String::new()));
            }
        }

        tracing::debug!("Installing the latest version ({}) for target {} to {}", latest.tag_name, target, location.display());

        let tempdir = tempfile::tempdir()?;

        let process = || {
            let archive = format!("jb_{}.tar.gz", target);

            let archive_path = tempdir.path().join(&archive);

            let url = format!(
                "{}/releases/download/{}/{archive}",
                env!("CARGO_PKG_REPOSITORY"),
                latest.tag_name,
            );

            jb::util::download(&url, &archive_path, None)?;
            jb::util::extract_archive(&archive_path, &tempdir.path().to_path_buf(), 0)?;

            // Delete current binary
            std::fs::remove_file(location)
                .with_context(|| format!("Failed to remove current binary at {}", location.display()))?;

            // Move new binary to location
            std::fs::rename(tempdir.path().join("jb"), location)
                .with_context(|| format!("Failed to move new binary to {}", location.display()))?;

            Ok::<(), anyhow::Error>(())
        };

        let output = process();

        tempdir.close()?;

        match output {
            Ok(()) => Ok((true, latest.body)),
            Err(e) => Err(e),
        }
    }
}