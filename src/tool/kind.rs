//! `JetBrains` tool kinds and parsing
//!
//! This module contains types and parsing for `JetBrains` tool kinds.

use std::fmt::Display;
use std::str::FromStr;
use serde::Serialize;
use crate::Tool;
use super::List;

/// The tool kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Kind {
    IntelliJIdeaUltimate,
    IntelliJIdeaCommunity,
    PyCharmProfessional,
    PyCharmCommunity,
    PhpStorm,
    GoLand,
    Rider,
    CLion,
    RustRover,
    WebStorm,
    RubyMine,
    DataGrip,
    DataSpell,
    Fleet,
    Aqua,
    Writerside,
    DotMemory,
    DotTrace,
    MPS,

    Space,
    Gateway,
}

impl Kind {
    /// Returns a list of all tool kinds.
    ///
    /// This is used for ordering and display purposes.
    /// The list is static and does not require any allocations.
    #[must_use]
    pub fn list() -> &'static [Self] {
        &[
            Self::IntelliJIdeaUltimate,
            Self::IntelliJIdeaCommunity,
            Self::PyCharmProfessional,
            Self::PyCharmCommunity,
            Self::PhpStorm,
            Self::GoLand,
            Self::Rider,
            Self::CLion,
            Self::RustRover,
            Self::WebStorm,
            Self::RubyMine,
            Self::DataGrip,
            Self::DataSpell,
            Self::Fleet,
            Self::Aqua,
            Self::Writerside,
            Self::DotMemory,
            Self::DotTrace,
            Self::MPS,
            Self::Space,
            Self::Gateway,
        ]
    }

    /// Get the binary name for this tool kind.
    ///
    /// This is used to determine the binary name for a tool kind and symbolically link it to the correct binary.
    #[must_use]
    pub fn binary(&self) -> &'static str {
        match self {
            Self::IntelliJIdeaUltimate | Self::IntelliJIdeaCommunity => "idea",
            Self::PyCharmProfessional | Self::PyCharmCommunity => "pycharm",
            Self::Fleet => "Fleet",
            _ => self.as_str(),
        }
    }

    /// Get the relative path to the executable for this tool kind.
    ///
    /// This is used to determine the binary path for a tool kind and symbolically link it to the correct binary.
    #[allow(clippy::single_match_else)]
    #[must_use]
    pub fn as_executable(&self) -> String {
        match self {
            Self::Fleet => format!("bin/{}", self.binary()),
            _ => format!("bin/{}.sh", self.binary()),
        }
    }

    /// Get the relative path to the icon for this tool kind.
    ///
    /// This is used to determine the icon path for a tool kind and symbolically link it to the correct icon.
    #[allow(clippy::single_match_else)]
    #[must_use]
    pub fn as_icon(&self) -> String {
        match self {
            Self::Fleet => format!("lib/{}.png", self.binary()),
            _ => format!("bin/{}.svg", self.binary()),
        }
    }

    /// Get the tool kind as a string (e.g. "intellij-idea-ultimate", "pycharm-professional").
    ///
    /// This returns the same string as the `FromStr` implementation.
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::IntelliJIdeaUltimate => "idea-ultimate",
            Self::IntelliJIdeaCommunity => "idea-community",
            Self::PyCharmProfessional => "pycharm-professional",
            Self::PyCharmCommunity => "pycharm-community",
            Self::PhpStorm => "phpstorm",
            Self::GoLand => "goland",
            Self::Rider => "rider",
            Self::CLion => "clion",
            Self::RustRover => "rustrover",
            Self::WebStorm => "webstorm",
            Self::RubyMine => "rubymine",
            Self::DataGrip => "datagrip",
            Self::DataSpell => "dataspell",
            Self::Fleet => "fleet",
            Self::Aqua => "aqua",
            Self::Writerside => "writerside",
            Self::DotMemory => "dotmemory",
            Self::DotTrace => "dottrace",
            Self::MPS => "mps",
            Self::Space => "space",
            Self::Gateway => "gateway",
        }
    }

    /// Get the tool kind as a code (e.g. "IIU", "IIC").
    ///
    /// This is used to fetch releases from `JetBrains`' API.
    #[must_use]
    pub fn code(&self) -> &'static str {
        match self {
            Self::IntelliJIdeaUltimate => "IIU",
            Self::IntelliJIdeaCommunity => "IIC",
            Self::PyCharmProfessional => "PCP",
            Self::PyCharmCommunity => "PCC",
            Self::PhpStorm => "PS",
            Self::GoLand => "GO",
            Self::Rider => "RD",
            Self::CLion => "CL",
            Self::RustRover => "RR",
            Self::WebStorm => "WS",
            Self::RubyMine => "RM",
            Self::DataGrip => "DG",
            Self::DataSpell => "DS",
            Self::Fleet => "FL",
            Self::Aqua => "QA",
            Self::Writerside => "WRS",
            Self::DotMemory => "DM",
            Self::DotTrace => "DP",
            Self::MPS => "MPS",

            Self::Space => "SPA",
            Self::Gateway => "GW",
        }
    }

    /// Get the tool kind as a human-readable description.
    ///
    /// These descriptions are directly from the `JetBrains` website.
    /// This is used to display a description in desktop entries.
    #[must_use]
    pub fn description(&self) -> &'static str {
        match self {
            Self::IntelliJIdeaCommunity | Self::IntelliJIdeaUltimate => "The Leading Java and Kotlin IDE",
            Self::PyCharmCommunity | Self::PyCharmProfessional => "The Python IDE for Professional Developers",
            Self::CLion => "A cross-platform IDE for C and C++",
            Self::RustRover => "Focus on what matters",
            Self::WebStorm => "The JavaScript and TypeScript IDE",
            Self::PhpStorm => "The Lightning-Smart PHP IDE",
            Self::Rider => "Fast & powerful cross-platform .NET IDE",
            Self::GoLand => "The complete IDE crafted for Gophers",
            Self::RubyMine => "Empowering Ruby Developers",
            Self::DataGrip => "Many databases, one tool",
            Self::DataSpell => "Turn data into insights with ease",
            Self::Fleet => "Next-generation IDE by JetBrains",
            Self::Aqua => "An IDE for writing tests you can be proud of",
            Self::Writerside => "Write, test, build, and publish the best documentation",
            Self::DotMemory => "The .NET Memory Profiler",
            Self::DotTrace => ".NET Performance Profiler",
            Self::MPS => "Meta Programming System",

            Self::Space => "The Intelligent Code Collaboration Platform",
            Self::Gateway => "Your single entry point to all remote development environments",
        }
    }

    /// Get the linked tool of this kind.
    ///
    /// This returns the linked tool of this kind, if any.
    ///
    /// # Errors
    /// This function will return an error if the tool list fails.
    pub fn linked(&self) -> anyhow::Result<Option<Tool>> {
        let tools = Tool::list_kind(*self)?;
        Ok(tools.into_iter().find(super::Link::is_linked))
    }

    /// Get the latest tool of this kind.
    ///
    /// This returns the latest tool of this kind, if any.
    ///
    /// # Errors
    /// This function will return an error if the tool list fails.
    pub fn latest(&self) -> anyhow::Result<Option<Tool>> {
        let tools = Tool::list_kind(*self)?;
        Ok(tools.into_iter().max())
    }
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::IntelliJIdeaUltimate => "IntelliJ IDEA Ultimate",
            Self::IntelliJIdeaCommunity => "IntelliJ IDEA Community",
            Self::PyCharmProfessional => "PyCharm Professional",
            Self::PyCharmCommunity => "PyCharm Community",
            Self::PhpStorm => "PhpStorm",
            Self::GoLand => "GoLand",
            Self::Rider => "Rider",
            Self::CLion => "CLion",
            Self::RustRover => "RustRover",
            Self::WebStorm => "WebStorm",
            Self::RubyMine => "RubyMine",
            Self::DataGrip => "DataGrip",
            Self::DataSpell => "DataSpell",
            Self::Fleet => "Fleet",
            Self::Aqua => "Aqua",
            Self::Writerside => "Writerside",
            Self::DotMemory => "dotMemory",
            Self::DotTrace => "dotTrace",
            Self::MPS => "MPS",
            Self::Space => "Space",
            Self::Gateway => "Gateway",
        })
    }
}

impl FromStr for Kind {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "idea-ultimate" => Ok(Self::IntelliJIdeaUltimate),
            "idea-community" => Ok(Self::IntelliJIdeaCommunity),
            "pycharm-professional" => Ok(Self::PyCharmProfessional),
            "pycharm-community" => Ok(Self::PyCharmCommunity),
            "phpstorm" => Ok(Self::PhpStorm),
            "goland" => Ok(Self::GoLand),
            "rider" => Ok(Self::Rider),
            "clion" => Ok(Self::CLion),
            "rustrover" => Ok(Self::RustRover),
            "webstorm" => Ok(Self::WebStorm),
            "rubymine" => Ok(Self::RubyMine),
            "datagrip" => Ok(Self::DataGrip),
            "dataspell" => Ok(Self::DataSpell),
            "fleet" => Ok(Self::Fleet),
            "aqua" => Ok(Self::Aqua),
            "writerside" => Ok(Self::Writerside),
            "dotmemory" => Ok(Self::DotMemory),
            "dottrace" => Ok(Self::DotTrace),
            "mps" => Ok(Self::MPS),
            "space" => Ok(Self::Space),
            "gateway" => Ok(Self::Gateway),
            _ => anyhow::bail!("Unknown tool kind: {}", s),
        }
    }
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        Self::list().iter().position(|kind| kind == self)
            .cmp(&Self::list().iter().position(|kind| kind == other))
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Serialize for Kind {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: serde::Serializer
    {
        self.as_str().serialize(serializer)
    }
}