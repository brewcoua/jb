use std::collections::HashMap;
use clap::builder::PossibleValue;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Release {
    pub date: String,

    #[serde(rename = "notesLink")]
    pub notes_link: Option<String>,
    #[serde(rename = "licenseRequired")]
    pub license_required: bool,

    pub version: String,
    #[serde(rename = "majorVersion")]
    pub major_version: String,
    pub build: String,

    pub downloads: HashMap<String, Download>,
}

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
    pub fn download(&self) -> Option<&Download> {
        let platform = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();

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

#[derive(Copy, Clone)]
pub enum ReleaseType {
    Release,
    EAP,
    Preview,
}

impl ReleaseType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Release => "release",
            Self::EAP => "eap",
            Self::Preview => "preview",
        }
    }
}

impl clap::ValueEnum for ReleaseType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Release, Self::EAP, Self::Preview]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Release => Some(PossibleValue::new("release")),
            Self::EAP => Some(PossibleValue::new("eap")),
            Self::Preview => Some(PossibleValue::new("preview")),
        }
    }
}