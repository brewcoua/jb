//! Module for deserializing responses from `JetBrains`' API.

use std::collections::HashMap;
use anyhow::bail;
use serde::Deserialize;
use crate::tool::{release, version::Version, build::Build};

/// The deserialized release data from `JetBrains`' API.
#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    #[serde(rename = "type")]
    pub release: release::Type,

    pub version: Version,
    pub build: Build,
    pub downloads: HashMap<String, Download>,
}

/// The deserialized download data from `JetBrains`' API. (This is a subset of the `Release` struct)
#[derive(Deserialize, Debug, Clone)]
pub struct Download {
    pub link: String,
    pub size: u64,

    #[serde(rename = "checksumLink")]
    pub checksum_link: String,
}

static ARCHITECTURES: &[&[&str]] = &[
    &["x86_64", "amd64", "x86", "x64"],
    &["aarch64", "arm64", "armv8l", "armv8b", "armv8", "armv9"],
    &["i386", "x86_32", "x86_32b", "x86_32l"],
];

impl Release {
    /// Returns the download for the current platform and architecture.
    ///
    /// # Errors
    /// This function will return an error if the download is not found, or if the platform or architecture is not supported.
    /// # Panics
    /// This function will panic if the platform or architecture is not valid.
    pub fn download(&self) -> anyhow::Result<Download> {
        let platform = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

        let platform = match platform.as_str() {
            "linux" => "linux",
            "macos" => "mac",
            "windows" => "windows",
            _ => anyhow::bail!("Unsupported platform {}", platform),
        };

        // Find the list of architectures that match the given architecture
        let archs = ARCHITECTURES
            .iter()
            .find(|archs| archs.contains(&arch.to_lowercase().as_str()));

        if archs.is_none() {
            bail!("Unsupported architecture {}", arch);
        }
        let archs = archs.unwrap();

        let arch_specific = archs
            .iter()
            .map(|arch| format!("{platform}_{arch}"))
            .chain(archs.iter().map(|arch| format!("{platform}-{arch}")))
            .chain(archs.iter().map(|arch| format!("{platform}{arch}")))
            .find(|arch| {
                self.downloads
                    .keys()
                    .any(|key| key.eq_ignore_ascii_case(arch))
            })
            .and_then(|arch| self.downloads.get(arch.as_str()));

        let platform_specific = self.downloads.get(platform);

        let download = match (arch_specific, platform_specific) {
            (Some(arch), _) => Some(arch),
            (_, Some(platform)) => Some(platform),
            _ => None,
        };

        download.cloned()
            .ok_or_else(|| anyhow::anyhow!("No download found for platform {}", platform))
    }
}