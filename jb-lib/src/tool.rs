//! Tools
//!
//! Tools can be installed, updated, and removed. They can also be used to perform actions.

use std::fmt::Display;
use std::str::FromStr;
use anyhow::Context;
use crate::env::Variable;

pub mod kind;
pub mod version;
pub mod build;
pub mod release;
pub mod action;

/// A tool.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[readonly::make]
pub struct Tool {
    pub kind: kind::Kind,
    pub version: Option<version::Version>,
    pub build: Option<build::Build>,
    pub release: Option<release::Type>,
}

impl Tool {
    #[must_use]
    pub fn new(kind: kind::Kind, version: Option<version::Version>, build: Option<build::Build>, release: Option<release::Type>) -> Self {
        Self { kind, version, build, release }
    }

    /// Returns the tool as a string.
    ///
    /// This is used to convert the tool to a string for display purposes and for serialization.
    #[must_use]
    pub fn as_str(&self) -> String {
        let mut s = self.kind.to_string();
        if self.version.is_some() || self.build.is_some() || self.release.is_some() {
            s.push_str("_");
        }

        if let Some(version) = &self.version {
            s.push_str(&format!("{version}"));
        }
        if let Some(build) = &self.build {
            s.push_str(&format!("{}{build}", if self.version.is_some() { "-" } else { "" }));
        }
        if let Some(release) = &self.release {
            s.push_str(&format!("{}{release}", if self.version.is_some() || self.build.is_some() { "-" } else { "" }));
        }

        s
    }

    /// Returns the path to the tool.
    ///
    /// This is used to convert the tool to a path for use in the filesystem.<br />
    /// **Note:** This does not check if the tool actually exists.
    pub fn as_path(&self) -> std::path::PathBuf {
        Variable::ToolsDirectory.get::<std::path::PathBuf>()
            .join(self.as_str())
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}",
               self.kind,
               if let Some(version) = &self.version { format!(" {}", version) } else { "".to_string() },
               if let Some(build) = &self.build { format!(" {}", build) } else { "".to_string() },
        )
    }
}

impl FromStr for Tool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Parse in the format of the as_str
        let mut parts = s.split('_');
        let kind = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("No tool kind found"))?
            .parse::<kind::Kind>()
            .with_context(|| "Failed to parse tool kind")?;

        let mut version = None;
        let mut build = None;
        let mut release = None;

        // First try parsing a version
        if let Ok(v) = parts.next().map(|v| v.parse::<version::Version>()) {
            version = v.ok();
        }

        // Then try parsing a build
        if let Ok(b) = parts.next().map(|b| b.parse::<build::Build>()) {
            build = b.ok();
        }

        // Then try parsing a release type
        if let Ok(r) = parts.next().map(|r| r.parse::<release::Type>()) {
            release = r.ok();
        }

        Ok(Self::new(kind, version, build, release))
    }
}
