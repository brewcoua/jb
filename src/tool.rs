//! Module for tools and tool actions.
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
        let mut s = self.kind.as_str().to_string();
        if self.version.is_some() || self.build.is_some() || self.release.is_some() {
            s.push('_');
        }

        if let Some(version) = &self.version {
            s.push_str(&format!("{version}"));
        }
        if let Some(build) = &self.build {
            s.push_str(&format!("{}{build}", if self.version.is_some() { "-" } else { "" }));
        }
        if let Some(release) = &self.release {
            s.push_str(&format!("{}{}", if self.version.is_some() || self.build.is_some() { "-" } else { "" }, release.as_str()));
        }

        s
    }

    /// Returns the path to the tool.
    ///
    /// This is used to convert the tool to a path for use in the filesystem.<br />
    /// **Note:** This does not check if the tool actually exists.
    #[must_use]
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

    /// Fills the tool with the latest version, build, and release.
    ///
    /// If the terminal is interactive, the user will be prompted to select a tool if multiple tools match.
    ///
    /// # Errors
    /// This function will return an error if no matching tool is found,
    /// Or if the prompt fails.
    pub fn fill(&self) -> anyhow::Result<Self> {
        let tools = Tool::list_kind(self.kind)?;

        let mut matching = vec![];
        for tool in tools {
            if self.matched(&tool) {
                matching.push(tool);
            }
        }

        if matching.is_empty() {
            anyhow::bail!("No matching tool found");
        }

        if matching.len() == 1 {
            return Ok(matching[0].clone());
        }

        // Sort by version, build, and release
        matching.sort();

        if atty::is(atty::Stream::Stdout) {
            // Prompt the user to select a tool
            let selected = inquire::Select::new("Select a tool", matching)
                .with_help_message("Use the arrow keys to navigate, and press Enter to select")
                .prompt()?;

            return Ok(selected);
        }

        Ok(matching[0].clone())
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}",
               self.kind,
               if let Some(version) = &self.version { format!(" {version}") } else { String::new() },
               if let Some(build) = &self.build { format!(" {build}") } else { String::new() },
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

        if let Some(part) = parts.next() {
            let mut parts = part.split('-');

            // First try parsing a version
            if let Some(Ok(v)) = parts.next().map(str::parse::<Version>) {
                version = Some(v);
            }

            // Then try parsing a build
            if let Some(Ok(b)) = parts.next().map(str::parse::<Build>) {
                build = Some(b);
            }

            // Then try parsing a release type
            if let Some(Ok(r)) = parts.next().map(str::parse::<Type>) {
                release = Some(r);
            }
        }

        Ok(Self::new(kind, version, build, release))
    }
}