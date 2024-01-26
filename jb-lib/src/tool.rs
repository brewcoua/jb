pub mod kind;
pub mod release;

use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use anyhow::{anyhow, bail, Context, Result};

use super::util::parse::{Download, Release};
use super::util::{file, sys};

pub use kind::Kind;
pub use release::{ReleaseType, ReleaseVersion};

/// A JetBrains tool
///
/// This struct represents a JetBrains tool, such as IntelliJ IDEA or PyCharm.
/// It contains information about the tool, such as its name, version, and installation directory.
/// However, it may not be installed yet.
#[derive(Debug, Clone, PartialEq)]
pub struct Tool {
    kind: Kind,
    version: Option<ReleaseVersion>,
    directory: Option<PathBuf>,
}


impl Tool {
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            version: None,
            directory: None,
        }
    }

    /// The default installation directory for JetBrains tools
    pub fn default_directory() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/JetBrains")
    }

    /// List all installed JetBrains tools
    ///
    /// # Panics
    /// If the apps directory does not exist or cannot be read
    /// If the apps directory contains a file that is not a directory
    pub fn list(directory: Option<&PathBuf>) -> Result<Vec<Tool>> {
        let directory = directory.cloned().unwrap_or(Self::default_directory());

        let tools = kind::Kind::list();
        let mut installed_tools: Vec<Tool> = Vec::new();

        for tool in tools.iter() {
            let apps_dir = directory.join("apps");
            let tool_dirs = std::fs::read_dir(&apps_dir)
                .with_context(|| format!("Failed to read apps directory {}", apps_dir.display()))?
                .filter_map(|entry| {
                    let entry = entry.expect("Failed to read entry");
                    let path = entry.path();

                    if path.is_dir() {
                        let name = path.file_name()?
                            .to_str()?;
                        if name.starts_with(tool.as_str()) {
                            let folder = path
                                .strip_prefix(&apps_dir)
                                .expect("Failed to strip apps directory")
                                .to_str()
                                .expect("Failed to convert path to string")
                                .to_string();
                            Some(folder)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            for tool_dir in tool_dirs {
                // remove only the first part of the tool name (e.g. "idea" from "idea-2021.1.1-eap")
                let tool_version = ReleaseVersion::from_str(
                    tool_dir
                        .strip_prefix(format!("{}-", tool.as_str()).as_str())
                        .with_context(|| format!("Failed to strip tool name prefix from {:?}", tool_dir))?
                );

                if tool_version.is_err() {
                    continue;
                }

                installed_tools.push(
                    Tool::new(tool.clone())
                        .with_version(tool_version.unwrap())
                        .with_directory(directory.clone())
                );
            }
        }

        Ok(installed_tools)
    }

    pub fn with_version(mut self, version: ReleaseVersion) -> Self {
        self.version = Some(version);
        self
    }

    pub fn with_directory(mut self, directory: PathBuf) -> Self {
        self.directory = Some(directory);
        self
    }

    pub fn kind(&self) -> Kind {
        self.kind
    }
    pub fn version(&self) -> Option<ReleaseVersion> {
        self.version
    }
    pub fn directory(&self) -> Option<&PathBuf> {
        self.directory.as_ref()
    }

    pub fn is_linked(&self) -> bool {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");
        let tool_dir = self.as_path();

        // Check the path inside the symlink for icons
        let icon_path = icons_dir.join(self.kind.as_str());
        // If it does not exist, the tool is not linked.
        if !icon_path.exists() {
            return false;
        }

        // Check the path inside the symlink for the tool
        let link = icon_path.read_link();
        if link.is_err() {
            return false;
        }

        link.unwrap()
            .starts_with(tool_dir)
    }

    pub fn link(&self) -> Result<()> {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");
        let tool_dir = self.as_path();

        // Create an icon symlink
        let icon_path = icons_dir.join(self.kind.as_str());
        let src_path = tool_dir.join(format!("bin/{}.svg", self.kind.src_name()));

        log::debug!("Linking {} to {}", self.kind.as_str(), src_path.display());

        // Delete the icon symlink, regardless of whether it exists or not
        std::fs::remove_file(&icon_path).ok();

        std::os::unix::fs::symlink(&src_path, &icon_path)
            .with_context(|| format!("Failed to link {} to {}", self.kind.as_str(), src_path.display()))?;
        log::debug!("Linked {} to {}", self.kind.as_str(), src_path.display());

        // Create a symlink to the latest version
        let binary_folder = sys::get_binary_dir()?;
        let binary_path = binary_folder.join(self.kind.as_str());

        let src_path = tool_dir.join(format!("bin/{}.sh", self.kind.src_name()));

        log::debug!("Linking {} to {}", self.kind.as_str(), src_path.display());

        // Delete the binary symlink, regardless of whether it exists or not
        std::fs::remove_file(&binary_path).ok();

        std::os::unix::fs::symlink(&src_path, &binary_path)
            .with_context(|| format!("Failed to link {} to {}", self.kind.as_str(), src_path.display()))?;
        log::debug!("Linked {} to {}", self.kind.as_str(), src_path.display());

        Ok(())
    }

    pub fn unlink(&self) -> Result<()> {
        if !self.is_linked() {
            bail!("{} is not linked", self.kind.as_str());
        }

        let directory = self.directory.clone().unwrap_or(Self::default_directory());

        // Try to find an alternative version to link
        let mut installed_tools = Self::list(self.directory.as_ref())?
            .into_iter()
            .filter(|installed_tool| installed_tool.kind == self.kind)
            .filter(|installed_tool| installed_tool.version != self.version)
            .collect::<Vec<_>>();

        installed_tools.sort_by_key(|installed_tool| installed_tool.version);

        let icons_dir = directory.join("icons");

        let icon_path = icons_dir.join(self.kind.as_str());
        let binary_path = sys::get_binary_dir()?.join(self.kind.as_str());

        // Delete the icon symlink
        std::fs::remove_file(&icon_path)
            .with_context(|| format!("Failed to remove icon symlink {}", icon_path.display()))?;

        // Delete the binary symlink
        std::fs::remove_file(&binary_path)
            .with_context(|| format!("Failed to remove binary symlink {}", binary_path.display()))?;

        if !installed_tools.is_empty() {
            // Link to the next version
            let fallback_tool = installed_tools.last().unwrap();
            let fallback_path = fallback_tool.as_path();

            log::debug!("Linking {} to {}", self.kind.as_str(), fallback_path.display());

            let src_path = fallback_path.join(format!("bin/{}.svg", self.kind.src_name()));
            std::os::unix::fs::symlink(&src_path, &icon_path)
                .with_context(|| format!("Failed to link {} to {}", self.kind.as_str(), src_path.display()))?;

            let src_path = fallback_path.join(format!("bin/{}.sh", self.kind.src_name()));
            std::os::unix::fs::symlink(&src_path, &binary_path)
                .with_context(|| format!("Failed to link {} to {}", self.kind.as_str(), src_path.display()))?;
        } else {
            log::debug!("No fallback version found for {}", self.kind.as_str());
        }

        Ok(())
    }

    pub fn name(&self) -> String {
        format!("{}-{}", self.kind.as_str(), self.version.unwrap_or(ReleaseVersion::default()).to_string())
    }

    pub fn as_path(&self) -> PathBuf {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let apps_dir = directory.join("apps");
        apps_dir.join(self.name())
    }

    pub fn download_link(&self) -> Result<Download> {
        let latest = self.version.is_none() || self.version.unwrap().is_latest();

        let release_type = if self.version.is_none() {
            self.kind.default_type()
        } else {
            self.version.unwrap().release
        };

        let url = format!(
            "https://data.services.jetbrains.com/products/releases?code={}&latest={}&type={}",
            self.kind.as_code(),
            latest,
            release_type.as_str()
        );

        let releases_by_code = reqwest::blocking::get(&url)
            .with_context(|| format!("Failed to fetch releases for {}, with URL {}", self.kind.as_str(), &url))?
            .json::<HashMap<String, Vec<Release>>>()
            .with_context(|| format!("Failed to parse releases for {}, with URL {}", self.kind.as_str(), &url))?;

        let releases = releases_by_code
            .get(&self.kind.as_code().to_string())
            .ok_or(anyhow!("Failed to find releases for {}", self.kind.as_str()))?;

        return if latest {
            // Loop through releases till we find a compatible download
            Ok(releases
                .iter()
                .find(|release| release.download().is_ok())
                .ok_or(anyhow!("Failed to find compatible download"))?
                .download()?
                .clone())
        } else {
            Ok(releases
                .iter()
                .filter(|release| release.version.compare_builds(&self.version.unwrap()) == Ordering::Equal)
                .find(|release| release.release_type == self.version.unwrap().release)
                .ok_or(anyhow!("Failed to find compatible download"))?
                .download()?
                .clone())
        };
    }

    pub fn install(&mut self) -> Result<()> {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");

        log::debug!("Fetching release '{}' for {}", self.version.unwrap_or(ReleaseVersion::default()), self.kind.as_str());
        let download = self.download_link()?;
        log::debug!("Found download for {} with version {}", self.kind.as_str(), download.version);

        self.version = Some(download.version.clone());

        let tool_dir = self.as_path();

        if tool_dir.exists() {
            bail!("{} is already installed to its latest version ({})", self.kind.as_str(), download.version);
        }

        log::debug!("Installing {} to {}", self.name(), tool_dir.display());

        std::fs::create_dir_all(&tool_dir)
            .with_context(|| format!("Failed to create tool directory {}", tool_dir.display()))?;
        std::fs::create_dir_all(&icons_dir)
            .with_context(|| format!("Failed to create icons directory {}", icons_dir.display()))?;

        let archive_name = download.link.split('/').last().expect("Failed to get download filename");

        let temp_folder = sys::mktemp_dir()?;

        log::debug!("Created temporary directory {}", temp_folder.display());

        let download_path = temp_folder.join(&archive_name);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(file::download_file(&download.link, &download_path, download.size))?;

        log::debug!("Extracting archive to {}", tool_dir.display());

        file::extract_archive(&download_path, &tool_dir, 1)?;

        log::debug!("Extracted {} to {}", download_path.display(), tool_dir.display());

        // Clean up the temporary directory
        std::fs::remove_dir_all(&temp_folder)
            .with_context(|| format!("Failed to remove temporary directory {}", temp_folder.display()))?;
        log::debug!("Removed temporary directory {}", temp_folder.display());


        // Symlink the tool
        self.link()?;
        log::debug!("Symlinked {} to {}", self.name(), tool_dir.display());

        Ok(())
    }

    pub fn uninstall(&self) -> Result<()> {
        let tool_dir = self.as_path();

        if !tool_dir.exists() {
            bail!("{} is not installed", self.name());
        }

        if self.is_linked() {
            log::debug!("Unlinking {}", self.name());
            self.unlink()?;
        }

        std::fs::remove_dir_all(&tool_dir)
            .with_context(|| format!("Failed to remove tool directory {}", tool_dir.display()))?;

        Ok(())
    }
}