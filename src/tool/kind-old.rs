//! The tool kind.
//!
//! The tool kind represents the tool among the `JetBrains` products.

use std::cmp::Ordering;
use std::str::FromStr;

use super::release::Type;
use clap::builder::PossibleValue;

/// Tool kind
///
/// This enum does not contain any information. It only represents which kind of tool it is.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Kind {
    IntelliJIdeaUltimate,
    IntelliJIdeaCommunity,
    PyCharmProfessional,
    PyCharmCommunity,
    PhpStorm,
    GoLand,
    Rider,
    CLion,
    ClionNova,
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
    /// This is used for display purposes and to loop over all tool kinds.
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
            Self::ClionNova,
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

    /// Returns the code for this tool kind.
    ///
    /// This is used to fetch the releases for this tool from the `JetBrains` website.
    #[must_use]
    pub fn as_code(&self) -> &str {
        match self {
            Self::IntelliJIdeaUltimate => "IIU",
            Self::IntelliJIdeaCommunity => "IIC",
            Self::PyCharmProfessional => "PCP",
            Self::PyCharmCommunity => "PCC",
            Self::PhpStorm => "PS",
            Self::GoLand => "GO",
            Self::Rider => "RD",
            Self::CLion => "CL",
            Self::ClionNova => "CLN",
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

    /// Returns the string representation of this tool kind.
    ///
    /// This is used to determine the directory name for the tool as well as the arguments for the CLI.
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
            Self::ClionNova => "clion-nova",
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

    /// Returns the pretty name for this tool kind.
    ///
    /// This is used for display purposes.
    #[must_use]
    pub fn pretty(&self) -> &'static str {
        match self {
            Self::IntelliJIdeaUltimate => "IntelliJ IDEA Ultimate",
            Self::IntelliJIdeaCommunity => "IntelliJ IDEA Community",
            Self::PyCharmProfessional => "PyCharm Professional",
            Self::PyCharmCommunity => "PyCharm Community",
            Self::PhpStorm => "PhpStorm",
            Self::GoLand => "GoLand",
            Self::Rider => "Rider",
            Self::CLion => "CLion",
            Self::ClionNova => "CLion Nova",
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
        }
    }

    /// Returns the source name for this tool kind.
    ///
    /// This is used to determine the binary name for the tool, as some tools may have the same binary name while being different tools. (e.g. `idea` for `IntelliJ IDEA Ultimate` and `IntelliJ IDEA Community`)
    #[must_use]
    pub fn src_name(&self) -> &str {
        match self {
            Self::IntelliJIdeaUltimate | Self::IntelliJIdeaCommunity => "idea",
            Self::PyCharmCommunity | Self::PyCharmProfessional => "pycharm",
            Self::ClionNova => "clion",
            _ => self.as_str(),
        }
    }

    /// Returns the default release type for this tool kind.
    ///
    /// This is used for tools that are not yet officially released.
    ///
    /// For example, the default release type for `Kind::Fleet` is `Type::Preview`.
    #[must_use]
    pub fn default_type(&self) -> Type {
        match self {
            Self::Fleet | Self::Aqua => Type::Preview,
            Self::Writerside | Self::ClionNova | Self::RustRover => Type::EAP,
            _ => Type::Release,
        }
    }

    /// Returns the tool kind by matching the beginning of the string, ignoring the rest.
    ///
    /// This is used to parse the tool from the CLI arguments, as it may be parsed alongside the version.
    /// # Errors
    /// This function returns an error if the string does not match any tool kind.
    pub fn from_str_lossy(s: &str) -> anyhow::Result<Self> {
        // Sort it by length to ensure that the longest match is found first
        let mut list = Self::list().to_vec();
        list.sort_by_key(|b| std::cmp::Reverse(b.as_str().len()));

        for kind in list {
            if s.starts_with(kind.as_str()) {
                return Ok(kind);
            }
        }
        anyhow::bail!("Unknown tool kind: {}", s)
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
            "clion-nova" => Ok(Self::ClionNova),
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
            _ => anyhow::bail!("Unknown tool kind: {}", s)
        }
    }
}

impl Ord for Kind {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for Kind {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}