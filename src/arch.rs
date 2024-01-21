use std::str::FromStr;

pub(crate) enum Architecture {
    x86_64,
    AArch64,
    i386,
    AArch32,
    rv64gc,
    ppc64le,
}

impl FromStr for Architecture {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x86_64" => Ok(Self::x86_64),
            "x86" => Ok(Self::x86_64),
            "amd64" => Ok(Self::x86_64),
            "AArch64" => Ok(Self::AArch64),
            "arm64" => Ok(Self::AArch64),
            "i386" => Ok(Self::i386),
            "AArch32" => Ok(Self::AArch32),
            "rv64gc" => Ok(Self::rv64gc),
            "ppc64le" => Ok(Self::ppc64le),
            _ => Err(format!("Unknown architecture: {}", s)),
        }
    }
}