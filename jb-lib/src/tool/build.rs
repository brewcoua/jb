//! Build version types and parsing
//!
//! This module contains types and parsing for build version numbers.

use std::fmt::Display;
use std::str::FromStr;
use anyhow::Context;

/// A build version number
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[readonly::make]
pub struct Build {
    pub major: u16,
    pub minor: u16,
    pub patch: Option<u16>,
}

impl Build {
    #[must_use]
    pub fn new(major: u16, minor: u16, patch: Option<u16>) -> Self {
        Self { major, minor, patch }
    }
}

impl Display for Build {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}{}", self.major, self.minor, self.patch.map_or("", |p| format!(".{}", p).as_str()))
    }
}

impl FromStr for Build {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.splitn(3, '.');
        let major = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No major version found"))?;
        let minor = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No minor version found"))?;

        let patch = parts
            .next();

        let major = major.parse::<u16>()
            .with_context(|| format!("Failed to parse major version: {}", major))?;
        let minor = minor.parse::<u16>()
            .with_context(|| format!("Failed to parse minor version: {}", minor))?;

        if let Some(patch) = patch {
            let patch = patch.parse::<u16>()
                .with_context(|| format!("Failed to parse patch version: {}", patch))?;
            Ok(Self::new(major, minor, Some(patch)))
        } else {
            Ok(Self::new(major, minor, None))
        }
    }
}