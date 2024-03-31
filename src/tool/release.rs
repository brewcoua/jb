use std::str::FromStr;
use std::fmt::Display;
use serde::{Deserialize, Serialize};
use super::kind::Kind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Type {
    Release,
    EAP,
    Preview,
}

impl Type {
    /// Returns the default release type for the given kind.
    ///
    /// This is used to determine the default release type for a tool kind, when the release type is not specified.
    #[must_use]
    pub fn kind_default(kind: Kind) -> Self {
        match kind {
            Kind::Fleet | Kind::Aqua => Self::Preview,
            Kind::Writerside | Kind::RustRover => Self::EAP,
            _ => Self::Release,
        }
    }

    /// Returns the release type as a string.
    ///
    /// This is used to convert the release type to a string for display purposes and for serialization.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Release => "release",
            Self::EAP => "eap",
            Self::Preview => "preview",
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Release => write!(f, "Release"),
            Self::EAP => write!(f, "EAP"),
            Self::Preview => write!(f, "Public Preview"),
        }
    }
}

impl FromStr for Type {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" => Ok(Self::Release),
            "eap" => Ok(Self::EAP),
            "preview" => Ok(Self::Preview),
            _ => anyhow::bail!("Failed to parse release type"),
        }
    }
}

impl<'de> Deserialize<'de> for Type {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for Type {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.as_str().serialize(serializer)
    }
}

impl Ord for Type {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self == other {
            return std::cmp::Ordering::Equal;
        }

        match (self, other) {
            (Self::Release, _) => std::cmp::Ordering::Greater,
            (_, Self::Release) => std::cmp::Ordering::Less,

            (Self::EAP, _) => std::cmp::Ordering::Greater,
            (_, Self::EAP) => std::cmp::Ordering::Less,

            (Self::Preview, _) => std::cmp::Ordering::Greater,
        }
    }
}

impl PartialOrd for Type {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}