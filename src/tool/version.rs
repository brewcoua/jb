//! Version types and parsing
//!
//! This module contains types and parsing for version numbers.

use std::fmt::Display;
use std::str::FromStr;
use anyhow::Context;

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
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.major, self.minor.map_or("", |m| format!(".{m}").as_str()))
    }
}

impl FromStr for Version {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Only split at the last dot
        let mut parts = s.rsplitn(2, '.');
        let major = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No major version found"))?;

        let minor = parts
            .next();

        let major = major.parse::<Major>()
            .with_context(|| format!("Failed to parse major version: {}", major))?;

        if let Some(minor) = minor {
            let minor = minor.parse::<u8>()
                .with_context(|| format!("Failed to parse minor version: {}", minor))?;
            Ok(Self::new(major, Some(minor)))
        } else {
            Ok(Self::new(major, None))
        }
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
