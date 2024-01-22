use clap::builder::PossibleValue;
use super::release::ReleaseType;

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn src_name(&self) -> &str {
        match self {
            Self::IntelliJIdeaUltimate => "idea",
            Self::IntelliJIdeaCommunity => "idea",
            Self::ClionNova => "clion",
            _ => self.as_str(),
        }
    }

    pub fn default_type(&self) -> ReleaseType {
        match self {
            Self::IntelliJIdeaUltimate => ReleaseType::Release,
            Self::IntelliJIdeaCommunity => ReleaseType::Release,
            Self::PyCharmProfessional => ReleaseType::Release,
            Self::PyCharmCommunity => ReleaseType::Release,
            Self::PhpStorm => ReleaseType::Release,
            Self::GoLand => ReleaseType::Release,
            Self::Rider => ReleaseType::Release,
            Self::CLion => ReleaseType::Release,
            Self::ClionNova => ReleaseType::EAP,
            Self::RustRover => ReleaseType::EAP,
            Self::WebStorm => ReleaseType::Release,
            Self::RubyMine => ReleaseType::Release,
            Self::DataGrip => ReleaseType::Release,
            Self::DataSpell => ReleaseType::Release,
            Self::Fleet => ReleaseType::Preview,
            Self::Aqua => ReleaseType::Preview,
            Self::Writerside => ReleaseType::EAP,
            Self::DotMemory => ReleaseType::Release,
            Self::DotTrace => ReleaseType::Release,
            Self::MPS => ReleaseType::Release,

            Self::Space => ReleaseType::Release,
            Self::Gateway => ReleaseType::Release,
        }
    }
}

impl clap::ValueEnum for Kind {
    fn value_variants<'a>() -> &'a [Self] {
        Self::list()
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(self.as_str()))
    }
}