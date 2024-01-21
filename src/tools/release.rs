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

    pub version: ReleaseVersion,
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

#[derive(Deserialize, Debug, Copy, Clone, PartialEq)]
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

    pub fn pretty(&self) -> &str {
        match self {
            Self::Release => "Release",
            Self::EAP => "EAP",
            Self::Preview => "Public Preview",
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

impl std::str::FromStr for ReleaseType {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" => Ok(Self::Release),
            "eap" => Ok(Self::EAP),
            "preview" => Ok(Self::Preview),
            _ => Err("Failed to parse release type"),
        }
    }
}

#[derive(Deserialize, Debug, Copy, Clone)]
pub struct ReleaseVersion {
    pub major: u32,
    pub minor: u32,
    pub patch: Option<u32>,
    pub release: ReleaseType,
}

impl ReleaseVersion {
    pub fn new(major: u32, minor: u32, patch: Option<u32>) -> Self {
        Self { major, minor, patch, release: ReleaseType::Release }
    }

    pub fn with_release(self, release: ReleaseType) -> Self {
        Self { release, ..self }
    }
}

impl std::fmt::Display for ReleaseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut version = format!("{}.{}", self.major, self.minor);
        if let Some(patch) = self.patch {
            version.push_str(&format!(".{}", patch));
        }
        if self.release != ReleaseType::Release {
            version.push_str(&format!("-{}", self.release.as_str()));
        }
        write!(f, "{}", version)
    }
}

impl std::str::FromStr for ReleaseVersion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut base = s.split('-');
        let mut parts = base.next().ok_or("Failed to parse version")?
            .split('.');

        // Parse version numbers
        let major = parts
            .next()
            .ok_or("Failed to parse major version")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse major version")?;

        let minor = parts
            .next()
            .ok_or("Failed to parse minor version")?
            .parse::<u32>()
            .map_err(|_| "Failed to parse minor version")?;

        let patch = parts
            .next()
            .map(|patch| patch.parse::<u32>().map_err(|_| "Failed to parse patch version"))
            .transpose()?;

        // Parse release type
        let release = base
            .next()
            .map(|release| match release {
                "release" => Ok(ReleaseType::Release),
                "eap" => Ok(ReleaseType::EAP),
                "preview" => Ok(ReleaseType::Preview),
                _ => Err("Failed to parse release type"),
            })
            .transpose()?;

        if let Some(release) = release {
            Ok(Self::new(major, minor, patch).with_release(release))
        } else {
            Ok(Self::new(major, minor, patch))
        }
    }
}