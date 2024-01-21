pub mod release;
pub mod install;

use std::collections::HashMap;
use clap::builder::PossibleValue;
use release::ReleaseType;

#[derive(Debug, Copy, Clone)]
pub enum Tool {
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

impl Tool {
    pub fn latest_release(&self, release_type: &ReleaseType) -> release::Release {
        let url = format!(
            "https://data.services.jetbrains.com/products/releases?code={}&latest=true&type={}",
            self.as_code(),
            release_type.as_str()
        );

        let releases_by_code: HashMap<String, Vec<release::Release>> = reqwest::blocking::get(&url)
            .expect("Failed to get releases by code")
            .json()
            .expect("Failed to parse releases by code");

        let releases = releases_by_code
            .get(&self.as_code().to_string())
            .expect("Failed to get releases");

        releases
            .first()
            .expect("Failed to get latest release")
            .clone()
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

    pub fn as_str(&self) -> &str {
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

    pub fn src_name(&self) -> &str {
        match self {
            Self::IntelliJIdeaUltimate => "idea",
            Self::IntelliJIdeaCommunity => "idea",
            Self::ClionNova => "clion",
            _ => self.as_str(),
        }
    }

    fn default_type(&self) -> ReleaseType {
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

impl clap::ValueEnum for Tool {
    fn value_variants<'a>() -> &'a [Self] {
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

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::IntelliJIdeaUltimate => Some(PossibleValue::new("intellij-idea-ultimate")),
            Self::IntelliJIdeaCommunity => Some(PossibleValue::new("intellij-idea-community")),
            Self::PyCharmProfessional => Some(PossibleValue::new("pycharm-professional")),
            Self::PyCharmCommunity => Some(PossibleValue::new("pycharm-community")),
            Self::PhpStorm => Some(PossibleValue::new("phpstorm")),
            Self::GoLand => Some(PossibleValue::new("goland")),
            Self::Rider => Some(PossibleValue::new("rider")),
            Self::CLion => Some(PossibleValue::new("clion")),
            Self::ClionNova => Some(PossibleValue::new("clion-nova")),
            Self::RustRover => Some(PossibleValue::new("rustrover")),
            Self::WebStorm => Some(PossibleValue::new("webstorm")),
            Self::RubyMine => Some(PossibleValue::new("rubymine")),
            Self::DataGrip => Some(PossibleValue::new("datagrip")),
            Self::DataSpell => Some(PossibleValue::new("dataspell")),
            Self::Fleet => Some(PossibleValue::new("fleet")),
            Self::Aqua => Some(PossibleValue::new("aqua")),
            Self::Writerside => Some(PossibleValue::new("writerside")),
            Self::DotMemory => Some(PossibleValue::new("dotmemory")),
            Self::DotTrace => Some(PossibleValue::new("dottrace")),
            Self::MPS => Some(PossibleValue::new("mps")),

            Self::Space => Some(PossibleValue::new("space")),
            Self::Gateway => Some(PossibleValue::new("gateway")),
        }
    }
}


