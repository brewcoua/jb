use std::cmp::Ordering;
use std::str::FromStr;
use clap::builder::PossibleValue;
use serde::Deserialize;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

impl<'de> Deserialize<'de> for ReleaseType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ReleaseType::from_str(&s).map_err(serde::de::Error::custom)
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

impl FromStr for ReleaseType {
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

impl Ord for ReleaseType {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Release, Self::Release) => Ordering::Equal,
            (Self::Release, _) => Ordering::Greater,
            (_, Self::Release) => Ordering::Less,

            (Self::EAP, Self::EAP) => Ordering::Equal,
            (Self::EAP, _) => Ordering::Greater,
            (_, Self::EAP) => Ordering::Less,

            (Self::Preview, Self::Preview) => Ordering::Equal,
            //(Self::Preview, _) => Ordering::Greater,
            //(_, Self::Preview) => Ordering::Less,
        }
    }
}

impl PartialOrd for ReleaseType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct ReleaseVersion {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub release: ReleaseType,
}

impl ReleaseVersion {
    pub fn new(major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Self {
        Self { major, minor, patch, release: ReleaseType::Release }
    }

    pub fn with_release(self, release: ReleaseType) -> Self {
        Self { release, ..self }
    }

    pub fn is_latest(&self) -> bool {
        self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }

    pub fn compare_builds(&self, other: &Self) -> Ordering {
        // Compare without checking release type
        if self.major.is_none() || other.major.is_none() {
            return Ordering::Equal;
        }

        let major = self.major.unwrap().cmp(&other.major.unwrap());

        if major != Ordering::Equal {
            return major;
        }

        if self.minor.is_none() || other.minor.is_none() {
            return Ordering::Equal;
        }

        let minor = self.minor.unwrap().cmp(&other.minor.unwrap());

        if minor != Ordering::Equal {
            return minor;
        }

        if self.patch.is_none() || other.patch.is_none() {
            return Ordering::Equal;
        }

        self.patch.unwrap().cmp(&other.patch.unwrap())
    }
}

impl Default for ReleaseVersion {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

impl<'de> Deserialize<'de> for ReleaseVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ReleaseVersion::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for ReleaseVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.major.is_none() && self.minor.is_none() && self.patch.is_none() {
            return if self.release == ReleaseType::Release {
                write!(f, "latest")
            } else {
                write!(f, "{}", self.release.as_str())
            }
        }

        let mut version = vec![self.major, self.minor, self.patch]
            .into_iter()
            .flatten()
            .map(|version| version.to_string())
            .collect::<Vec<_>>()
            .join(".");

        if self.release != ReleaseType::Release {
            version.push_str(&format!("-{}", self.release.as_str()));
        }
        write!(f, "{}", version)
    }
}

impl FromStr for ReleaseVersion {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut base = s.split('-');

        let first = base.next().ok_or("Failed to parse version")?;

        // Check if it's a release type
        if let Ok(release) = ReleaseType::from_str(first) {
            return Ok(Self::new(None, None, None).with_release(release));
        }

        let parts = first
            .split('.')
            .map(|part| {
                if part.is_empty() {
                    return Ok(None);
                }

                part.parse::<u32>()
                    .map_err(|_| "Failed to parse version number")
                    .map(Some)
            })
            .collect::<Result<Vec<_>, _>>()?;

        if parts.len() > 3 {
            return Err("Failed to parse version: too many version numbers");
        }

        if parts.first().is_none() {
            return Err("Failed to parse version: major version is required if no release_type is specified");
        }

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

        Ok(Self {
            major: parts.first().cloned().flatten(),
            minor: parts.get(1).cloned().flatten(),
            patch: parts.get(2).cloned().flatten(),
            release: release.unwrap_or(ReleaseType::Release),
        })
    }
}