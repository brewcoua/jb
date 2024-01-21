use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Release {
    pub(crate) date: String,

    #[serde(rename = "notesLink")]
    pub(crate) notes_link: Option<String>,
    #[serde(rename = "licenseRequired")]
    pub(crate) license_required: bool,

    pub(crate) version: String,
    #[serde(rename = "majorVersion")]
    pub(crate) major_version: String,
    pub(crate) build: String,

    pub(crate) downloads: HashMap<String, Download>,
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct Download {
    pub(crate) link: String,
    pub(crate) size: u64,
    #[serde(rename = "checksumLink")]
    pub(crate) checksum_link: String,
}

static ARCHITECTURES: &[&[&str]] = &[
    &["x86_64", "amd64", "x86", "x64"],
    &["aarch64", "arm64", "armv8l", "armv8b", "armv8", "armv9"],
    &["i386", "x86_32", "x86_32b", "x86_32l"],
];

impl Release {
    pub fn download(&self, platform: Option<String>, arch: Option<String>) -> Option<&Download> {
        let platform = platform.unwrap_or_else(|| std::env::consts::OS.to_string());
        let arch = arch.unwrap_or_else(|| std::env::consts::ARCH.to_string());

        let platform = match platform.as_str() {
            "linux" => "linux",
            "macos" => "mac",
            "windows" => "windows",
            _ => return None,
        };

        // Find the list of architectures that match the given architecture
        let archs = ARCHITECTURES
            .iter()
            .find(|archs| archs.contains(&arch.to_lowercase().as_str()))
            .expect("Failed to find architectures matching the given architecture");

        let arch_specific = archs
            .iter()
            .map(|arch| format!("{}_{}", platform, arch))
            .chain(archs.iter().map(|arch| format!("{}-{}", platform, arch)))
            .chain(archs.iter().map(|arch| format!("{}{}", platform, arch)))
            .find(|arch| self.downloads.keys().any(|key| key.eq_ignore_ascii_case(arch)))
            .map(|arch| self.downloads.get(arch.as_str()))
            .flatten();

        let platform_specific = self.downloads.get(platform);

        match (arch_specific, platform_specific) {
            (Some(arch), _) => Some(arch),
            (_, Some(platform)) => Some(platform),
            _ => None,
        }
    }
}