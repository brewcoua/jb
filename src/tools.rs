mod release;

use std::collections::HashMap;
use clap::builder::PossibleValue;

#[derive(Debug, Copy, Clone)]
pub(crate) enum Tool {
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
    dotMemory,
    dotTrace,
    MPS,

    Space,
    Gateway,
}

#[derive(Copy, Clone)]
pub(crate) enum ReleaseType {
    Release,
    EAP,
    Preview,
}

impl Tool {
    pub(crate) fn latest_release(&self, release_type: Option<ReleaseType>, latest: Option<bool>) -> release::Release {
        let release_type = release_type.unwrap_or_else(|| self.default_type());
        let latest = latest.unwrap_or(true);

        let url = format!(
            "https://data.services.jetbrains.com/products/releases?code={}&latest={}&type={}",
            self.as_code(),
            latest,
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

    pub(crate) fn as_code(&self) -> &str {
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
            Self::dotMemory => "DM",
            Self::dotTrace => "DP",
            Self::MPS => "MPS",

            Self::Space => "SPA",
            Self::Gateway => "GW",
        }
    }

    pub(crate) fn as_str(&self) -> &str {
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
            Self::dotMemory => "dotmemory",
            Self::dotTrace => "dottrace",
            Self::MPS => "mps",

            Self::Space => "space",
            Self::Gateway => "gateway",
        }
    }

    pub(crate) fn src_name(&self) -> &str {
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
            Self::dotMemory => ReleaseType::Release,
            Self::dotTrace => ReleaseType::Release,
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
            Self::dotMemory,
            Self::dotTrace,
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
            Self::dotMemory => Some(PossibleValue::new("dotmemory")),
            Self::dotTrace => Some(PossibleValue::new("dottrace")),
            Self::MPS => Some(PossibleValue::new("mps")),

            Self::Space => Some(PossibleValue::new("space")),
            Self::Gateway => Some(PossibleValue::new("gateway")),
        }
    }
}

impl ReleaseType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Release => "release",
            Self::EAP => "eap",
            Self::Preview => "preview",
        }
    }
}

impl clap::ValueEnum for ReleaseType {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Release, Self::EAP, Self::Preview]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            Self::Release => Some(PossibleValue::new("release")),
            Self::EAP => Some(PossibleValue::new("eap")),
            Self::Preview => Some(PossibleValue::new("preview")),
        }
    }
}
