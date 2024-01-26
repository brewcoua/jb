use std::collections::HashMap;
use serde::Deserialize;
use anyhow::{bail, Result};
use crate::tool::release::{ReleaseType, ReleaseVersion};

#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    pub date: String,

    #[serde(rename = "type")]
    pub release_type: ReleaseType,

    #[serde(rename = "notesLink")]
    pub notes_link: Option<String>,
    #[serde(rename = "licenseRequired")]
    pub license_required: Option<bool>,

    pub version: ReleaseVersion,
    #[serde(rename = "majorVersion")]
    pub major_version: String,
    pub build: String,

    pub downloads: HashMap<String, DownloadRaw>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DownloadRaw {
    pub link: String,
    pub size: u64,
    #[serde(rename = "checksumLink")]
    pub checksum_link: String,
}

#[derive(Debug, Clone)]
pub struct Download {
    pub version: ReleaseVersion,
    pub link: String,
    pub size: u64,
    pub checksum_link: String,
}

static ARCHITECTURES: &[&[&str]] = &[
    &["x86_64", "amd64", "x86", "x64"],
    &["aarch64", "arm64", "armv8l", "armv8b", "armv8", "armv9"],
    &["i386", "x86_32", "x86_32b", "x86_32l"],
];

impl Release {
    pub fn download(&self) -> Result<Download> {
        let platform = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

        let platform = match platform.as_str() {
            "linux" => "linux",
            "macos" => "mac",
            "windows" => "windows",
            _ => bail!("Unsupported platform {}", platform)
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
            .map(|arch| format!("{}_{}", platform, arch))
            .chain(archs.iter().map(|arch| format!("{}-{}", platform, arch)))
            .chain(archs.iter().map(|arch| format!("{}{}", platform, arch)))
            .find(|arch| self.downloads.keys().any(|key| key.eq_ignore_ascii_case(arch)))
            .map(|arch| self.downloads.get(arch.as_str()))
            .flatten();

        let platform_specific = self.downloads.get(platform);

        let download_raw = match (arch_specific, platform_specific) {
            (Some(arch), _) => Some(arch),
            (_, Some(platform)) => Some(platform),
            _ => None,
        };

        let result = download_raw.map(|download| Download {
            version: self.version.clone().with_release(self.release_type.clone()),
            link: download.link.clone(),
            size: download.size,
            checksum_link: download.checksum_link.clone(),
        });

        return if result.is_some() {
            Ok(result.unwrap())
        } else {
            bail!("No download found for platform {} and architecture {}", platform, arch);
        }
    }
}