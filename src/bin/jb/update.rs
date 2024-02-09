//! Module for updating the CLI.

use std::path::PathBuf;
use anyhow::Context;
use serde::Deserialize;
use crate::emoji::{LOOKING_GLASS,TAG,DOWNLOAD};

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
    pub fn try_update(location: &PathBuf, force: bool) -> anyhow::Result<String> {
        jb::info!("{LOOKING_GLASS} Checking for updates...");

        let latest = Self::latest()?;
        let current = env!("CARGO_PKG_VERSION");
        let target = env!("TARGET");

        jb::info!("{TAG} Found latest version: {} (current: v{current})", latest.tag_name);

        if latest.tag_name == format!("v{current}") {
            if force {
                jb::warn!("The latest version is already installed, forcing the update...");
            } else {
                anyhow::bail!("The latest version is already installed ({current})");
            }
        }

        jb::debug!("Installing the latest version ({}) for target {} to {}", latest.tag_name, target, location.display());

        jb::info!("{DOWNLOAD} Downloading the latest version...");

        let tempdir = tempfile::tempdir()?;

        let process = || {
            let archive = format!("jb_{target}.tar.gz");

            let url = format!(
                "{}/releases/download/{}/{archive}",
                env!("CARGO_PKG_REPOSITORY"),
                latest.tag_name,
            );

            jb::util::download_extract(&url, &tempdir.path().to_path_buf(), None, None)?;

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
            Ok(()) => Ok(latest.body),
            Err(e) => Err(e),
        }
    }
}