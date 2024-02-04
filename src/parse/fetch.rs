use std::collections::HashMap;
use anyhow::Context;

use crate::tool::{Tool, release};
use super::deserial::{Release, Download};

#[derive(Debug, Clone)]
pub struct Fetch {
    pub tool: Tool,
    pub download: Download,
}

pub fn release(tool: &Tool) -> anyhow::Result<Fetch> {
    let latest = tool.version.is_none() && tool.build.is_none();
    let release = tool.release.unwrap_or(release::Type::kind_default(tool.kind));

    let url = format!(
        "https://data.services.jetbrains.com/products/releases?code={}&latest={}&type={}",
        tool.kind.code(),
        latest,
        release.as_str()
    );

    let releases = reqwest::blocking::get(&url)
        .with_context(|| format!("Failed to fetch releases for {}", tool.as_str()))?
        .json::<HashMap<String, Vec<Release>>>()
        .with_context(|| format!("Failed to parse releases for {}", tool.as_str()))?;

    let releases = releases.get(tool.kind.code())
        .with_context(|| format!("No releases found for {}", tool.as_str()))?;

    let release = if latest {
        releases.first()
            .with_context(|| format!("No releases found for {}", tool.as_str()))?
    } else {
        releases.iter()
            .find(|release| {
                if (tool.version.is_some() && &release.version != tool.version.as_ref().unwrap()) ||
                    (tool.build.is_some() && &release.build != tool.build.as_ref().unwrap()) {
                    return false;
                }
                true
            })
            .with_context(|| format!("No release found for {}", tool.as_str()))?
    };

    let download = release.download()
        .with_context(|| format!("Failed to fetch download for {}", tool.as_str()))?;

    let release = release.clone();

    Ok(Fetch {
        tool: Tool::new(
            tool.kind,
            Some(release.version),
            Some(release.build),
            Some(release.release),
        ),
        download,
    })
}