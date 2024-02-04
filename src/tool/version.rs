//! Version types and parsing
//!
//! This module contains types and parsing for version numbers.

use std::fmt::Display;
use std::str::FromStr;
use anyhow::Context;
use serde::Deserialize;

/// A version number
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[readonly::make]
pub struct Version {
    /// The major version (e.g. 2021.1)
    pub major: Major,
    /// The minor version (if any)
    pub minor: Option<u8>,
}

/// A major version number (e.g. 2021.1)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[readonly::make]
pub struct Major {
    pub year: u16,
    pub month: u8,
}

impl Version {
    #[must_use]
    pub fn new(major: Major, minor: Option<u8>) -> Self {
        Self { major, minor }
    }

    /// Returns whether the version matches another version.
    ///
    /// This is used to check if a version matches another version, used in arguments and commands.
    #[must_use]
    pub fn matched(&self, other: &Version) -> bool {
        if self.major != other.major {
            return false;
        }

        if let Some(minor) = self.minor {
            if other.minor.is_none() || minor != other.minor.unwrap() {
                return false;
            }
        }

        true
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.major, self.minor.map_or("".to_string(), |m| format!(".{m}")))
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If it only has 2 parts, only the major version is present
        let parts = s.split('.').collect::<Vec<_>>();

        if parts.len() < 2 || parts.len() > 3 {
            anyhow::bail!("Invalid version: {}", s);
        }

        let major = Major::from_str(format!("{}.{}", parts[0], parts[1]).as_str())
            .with_context(|| format!("Failed to parse major version: {}", parts[..1].join(".")))?;

        if parts.len() == 2 {
            return Ok(Self::new(major, None));
        }

        // If it has 3 parts, both the major and minor version are present
        let minor = parts[2].parse::<u8>()
            .with_context(|| format!("Failed to parse minor version: {}", parts[2]))?;
        return Ok(Self::new(major, Some(minor)));
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Major {
    #[must_use]
    pub fn new(year: u16, month: u8) -> Self {
        Self { year, month }
    }
}

impl Display for Major {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}", self.year, self.month)
    }
}

impl FromStr for Major {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(2, '.');
        let year = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No year found"))?;
        let month = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No month found"))?;

        let year = year.parse::<u16>()
            .with_context(|| format!("Failed to parse year: {}", year))?;
        let month = month.parse::<u8>()
            .with_context(|| format!("Failed to parse month: {}", month))?;

        Ok(Self::new(year, month))
    }
}
