//! `JetBrains` tool kinds and parsing
//!
//! This module contains types and parsing for `JetBrains` tool kinds.

use std::fmt::Display;
use std::str::FromStr;

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
    CLionNova,
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
            Self::CLionNova,
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
            Self::CLionNova => "clion",
            Self::Fleet => "Fleet",
            _ => self.as_str(),
        }
    }

    /// Get the relative path to the executable for this tool kind.
    ///
    /// This is used to determine the binary path for a tool kind and symbolically link it to the correct binary.
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
            Self::CLionNova => "clion-nova",
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
            Self::CLionNova => "CLN",
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
            Self::CLionNova => "CLion Nova",
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
            "clion-nova" => Ok(Self::CLionNova),
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