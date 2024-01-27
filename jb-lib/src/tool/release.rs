//! Release version parsing and comparison

use anyhow::{anyhow, Result};
use clap::builder::PossibleValue;
use serde::Deserialize;
use std::cmp::Ordering;
use std::str::FromStr;

/// Release type
///
/// `JetBrains` has three release types:
/// - Release
/// - EAP
/// - Public Preview
///
/// This enum represents those release types.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Type {
    Release,
    EAP,
    Preview,
}

impl Type {
    /// Get the release type as a string (e.g. "release", "eap", "preview").
    ///
    /// This is fully static and does not require any allocations.
    #[must_use]
    pub fn as_str(&self) -> &str {
        match self {
            Self::Release => "release",
            Self::EAP => "eap",
            Self::Preview => "preview",
        }
    }

    /// Get the release type as a pretty string (e.g. "Release", "EAP", "Public Preview")
    ///
    /// This is fully static and does not require any allocations.
    #[must_use]
    pub fn pretty(&self) -> &str {
        match self {
            Self::Release => "Release",
            Self::EAP => "EAP",
            Self::Preview => "Public Preview",
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Type::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl clap::ValueEnum for Type {
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

impl FromStr for Type {
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

impl Ord for Type {
    fn cmp(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }

        match (self, other) {
            (Self::Release, _) => Ordering::Greater,
            (_, Self::Release) => Ordering::Less,

            (Self::EAP, _) => Ordering::Greater,
            (_, Self::EAP) => Ordering::Less,

            (Self::Preview, _) => Ordering::Greater,
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Release version
///
/// This struct represents a release version, including the release type.
///
/// It can be parsed from a string, and can be compared to other release versions.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    pub major: Option<u32>,
    pub minor: Option<u32>,
    pub patch: Option<u32>,
    pub release: Type,
}

impl Version {
    #[must_use]
    pub fn new(major: Option<u32>, minor: Option<u32>, patch: Option<u32>) -> Self {
        Self {
            major,
            minor,
            patch,
            release: Type::Release,
        }
    }

    /// Set the release type
    ///
    /// This returns a new `ReleaseVersion` with the release type set to the specified value.
    ///
    /// # Example
    /// ```rust
    /// use jb_lib::tool::{Type, Version};
    ///
    /// let version = Version::new(Some(2021), Some(2), Some(3));
    /// assert_eq!(version.release, Type::Release);
    ///
    /// let version = version.with_release(Type::EAP);
    /// assert_eq!(version.release, Type::EAP);
    /// ```
    #[must_use]
    pub fn with_release(self, release: Type) -> Self {
        Self { release, ..self }
    }

    /// Check if this version is the latest version
    ///
    /// This returns `true` if the version represents the latest version, and `false` otherwise.
    /// This only checks if all fields are `None`, and does not check the release type.
    ///
    /// # Example
    /// ```rust
    /// use jb_lib::tool::Version;
    ///
    /// let version = Version::new(None, None, None);
    /// assert!(version.is_latest());
    /// ```
    #[must_use]
    pub fn is_latest(&self) -> bool {
        self.major.is_none() && self.minor.is_none() && self.patch.is_none()
    }

    /// Compare this version to another version, ignoring the release type
    ///
    /// This returns an `Ordering` representing the comparison between the two versions.
    /// This does not check the release type, and only compares the version numbers.
    ///
    /// # Errors
    /// This returns an error if any of the version numbers are missing when comparing.
    /// This should never happen, as the version numbers are always set when parsing or an early return is made.
    ///
    /// # Example
    /// ```rust
    /// use std::cmp::Ordering;
    /// use jb_lib::tool::Version;
    ///
    /// let version = Version::new(Some(2021), Some(2), Some(3));
    /// let other = Version::new(Some(2021), Some(2), Some(4));
    /// assert_eq!(version.compare_builds(&other).unwrap(), Ordering::Less);
    /// ```
    pub fn compare_builds(&self, other: &Self) -> Result<Ordering> {
        if self.major.is_none() || other.major.is_none() {
            return Ok(Ordering::Equal);
        }

        let major = self
            .major
            .ok_or(anyhow!(
                "Failed to compare builds: major version is missing"
            ))?
            .cmp(&other.major.ok_or(anyhow!(
                "Failed to compare builds: major version is missing"
            ))?);

        if major != Ordering::Equal {
            return Ok(major);
        }

        if self.minor.is_none() || other.minor.is_none() {
            return Ok(Ordering::Equal);
        }

        let minor = self
            .minor
            .ok_or(anyhow!(
                "Failed to compare builds: minor version is missing"
            ))?
            .cmp(&other.minor.ok_or(anyhow!(
                "Failed to compare builds: minor version is missing"
            ))?);

        if minor != Ordering::Equal {
            return Ok(minor);
        }

        if self.patch.is_none() || other.patch.is_none() {
            return Ok(Ordering::Equal);
        }

        Ok(self
            .patch
            .ok_or(anyhow!(
                "Failed to compare builds: patch version is missing"
            ))?
            .cmp(&other.patch.ok_or(anyhow!(
                "Failed to compare builds: patch version is missing"
            ))?))
    }
}

impl Default for Version {
    fn default() -> Self {
        Self::new(None, None, None)
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Version::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.major.is_none() && self.minor.is_none() && self.patch.is_none() {
            return if self.release == Type::Release {
                write!(f, "latest")
            } else {
                write!(f, "{}", self.release.as_str())
            };
        }

        let mut version = vec![self.major, self.minor, self.patch]
            .into_iter()
            .flatten()
            .map(|version| version.to_string())
            .collect::<Vec<_>>()
            .join(".");

        if self.release != Type::Release {
            version.push_str(&format!("-{}", self.release.as_str()));
        }
        write!(f, "{version}")
    }
}

impl FromStr for Version {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut base = s.split('-');

        let first = base.next().ok_or("Failed to parse version")?;

        // Check if it's a release type
        if let Ok(release) = Type::from_str(first) {
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
                "release" => Ok(Type::Release),
                "eap" => Ok(Type::EAP),
                "preview" => Ok(Type::Preview),
                _ => Err("Failed to parse release type"),
            })
            .transpose()?;

        Ok(Self {
            major: parts.first().copied().flatten(),
            minor: parts.get(1).copied().flatten(),
            patch: parts.get(2).copied().flatten(),
            release: release.unwrap_or(Type::Release),
        })
    }
}
