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

pub use action::*;
pub use kind::Kind;
pub use version::Version;
pub use build::Build;
pub use release::Type;

/// A tool.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
#[readonly::make]
pub struct Tool {
    pub kind: Kind,
    pub version: Option<Version>,
    pub build: Option<Build>,
    pub release: Option<Type>,
}

impl Tool {
    #[must_use]
    pub fn new(kind: Kind, version: Option<Version>, build: Option<Build>, release: Option<Type>) -> Self {
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

    /// Returns whether the tool matches another tool.
    ///
    /// This is used in arguments to match multiple tools.
    #[must_use]
    pub fn matched(&self, other: &Self) -> bool {
        // Match the kind
        if self.kind != other.kind {
            return false;
        }

        // Match the version
        if let Some(version) = &self.version {
            if let Some(other_version) = &other.version {
                if !version.matched(other_version) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Match the build
        if let Some(build) = &self.build {
            if let Some(other_build) = &other.build {
                if !build.matched(other_build) {
                    return false;
                }
            } else {
                return false;
            }
        }

        // Match the release
        if let Some(release) = &self.release {
            if let Some(other_release) = &other.release {
                if release != other_release {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
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
            .parse::<Kind>()
            .with_context(|| "Failed to parse tool kind")?;

        let mut version = None;
        let mut build = None;
        let mut release = None;

        // First try parsing a version
        if let Some(Ok(v)) = parts.next().map(|v| v.parse::<Version>()) {
            version = Some(v);
        }

        // Then try parsing a build
        if let Some(Ok(b)) = parts.next().map(|b| b.parse::<Build>()) {
            build = Some(b);
        }

        // Then try parsing a release type
        if let Some(Ok(r)) = parts.next().map(|r| r.parse::<Type>()) {
            release = Some(r);
        }

        Ok(Self::new(kind, version, build, release))
    }
}
