use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;
use crate::tools::release::{ReleaseType, ReleaseVersion};
use crate::utils::parsing::{Download, Release};
use crate::utils::{file, sys};

pub mod release;
pub mod install;
pub mod list;
pub mod kind;

/// A JetBrains tool
///
/// This struct represents a JetBrains tool, such as IntelliJ IDEA or PyCharm.
/// It contains information about the tool, such as its name, version, and installation directory.
/// However, it may not be installed yet.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Tool {
    kind: kind::Kind,
    version: Option<ReleaseVersion>,
    directory: Option<PathBuf>,
}


impl Tool {
    pub fn new(kind: kind::Kind) -> Self {
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
    pub fn list(directory: Option<PathBuf>) -> Result<Vec<Tool>, &'static str> {
        let directory = directory.unwrap_or(Self::default_directory());

        let tools = kind::Kind::list();
        let mut installed_tools: Vec<Tool> = Vec::new();

        for tool in tools.iter() {
            let apps_dir = directory.join("apps");
            let tool_dirs = std::fs::read_dir(&apps_dir)
                .expect("Failed to read apps directory")
                .filter_map(|entry| {
                    let entry = entry.expect("Failed to read entry");
                    let path = entry.path();

                    if path.is_dir() {
                        let name = path.file_name().expect("Failed to get file name").to_str().expect("Failed to convert file name to string");
                        if name.starts_with(tool.as_str()) {
                            let folder = path
                                .strip_prefix(&apps_dir)
                                .expect("Failed to strip directory prefix")
                                .to_str()
                                .expect("Failed to convert tool name to string")
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
                let tool_version = release::ReleaseVersion::from_str(
                    tool_dir
                        .strip_prefix(format!("{}-", tool.as_str()).as_str())
                        .expect("Failed to strip tool name prefix")
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

    pub fn is_linked(&self) -> bool {
        let directory = self.directory.unwrap_or(Self::default_directory());
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

    pub fn link(&self) -> Result<(), Box<dyn std::error::Error>> {
        let directory = self.directory.unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");
        let tool_dir = self.as_path();

        // Create an icon symlink
        let icon_path = icons_dir.join(self.kind.as_str());
        let src_path = tool_dir.join(format!("bin/{}.svg", self.kind.src_name()));

        if icon_path.exists() {
            std::fs::remove_file(&icon_path)?;
        }

        std::os::unix::fs::symlink(&src_path, &icon_path)?;

        // Create a symlink to the latest version
        let binary_folder = sys::get_binary_dir()?;
        let binary_path = binary_folder.join(self.kind.as_str());

        if binary_path.exists() {
            std::fs::remove_file(&binary_path)?;
        }

        let src_path = tool_dir.join(format!("bin/{}.sh", self.kind.src_name()));

        std::os::unix::fs::symlink(&src_path, &binary_path)?;

        Ok(())
    }

    pub fn as_path(&self) -> PathBuf {
        let directory = self.directory.unwrap_or(Self::default_directory());
        let apps_dir = directory.join("apps");
        apps_dir.join(format!("{}-{}", self.kind.as_str(), self.version.unwrap().to_string()))
    }

    pub fn download_link(&self) -> Result<Download, Box<dyn std::error::Error>> {
        let latest = self.version.is_none() || self.version.unwrap().is_latest();

        let url = format!(
            "https://data.services.jetbrains.com/products/releases?code={}&latest={}&type={}",
            self.as_code(),
            latest,
            self.version.unwrap_or(ReleaseVersion::default()).release.as_str()
        );

        let releases_by_code = reqwest::blocking::get(&url)?
            .json::<serde_json::Value>()?;

        let releases_by_code: HashMap<String, Vec<Release>> = releases_by_code
            .deserialize::<HashMap<String, Vec<Release>>>()?;

        let releases = releases_by_code
            .get(&self.as_code().to_string())?;

        return if latest {
            // Loop through releases till we find a compatible download
            releases
                .iter()
                .find(|release| release.download().is_some())?
                .download()?
                .clone()
        } else {
            releases
                .iter()
                .filter(|release| release.version.compare_builds(&self.version.unwrap()) == Ordering::Equal)
                .find(|release| release.release_type == self.version.unwrap().release)?
                .download()?
                .clone()
        }
    }

    pub fn install(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let directory = self.directory.unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");

        log::debug!("Fetching release '{}' for {}", self.version.unwrap_or(ReleaseVersion::default()), self.kind.as_str());
        let download = self.download_link()?;
        log::debug!("Found download for {} with version {}", self.kind.as_str(), download.version);

        self.version = Some(download.version.clone());

        let tool_dir = self.as_path();

        if tool_dir.exists() {
            return Err(format!("{} is already installed to its latest version ({})", self.kind.as_str(), download.version).into());
        }

        log::info!("Installing {} to {}", self.kind.as_str(), tool_dir.display());

        std::fs::create_dir_all(&tool_dir)?;
        std::fs::create_dir_all(&icons_dir)?;

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
        std::fs::remove_dir_all(&temp_folder)?;
        log::debug!("Removed temporary directory {}", temp_folder.display());


        // Symlink the tool
        self.link()?;
        log::info!("Installed {} to {}", self.kind.as_str(), tool_dir.display());

        Ok(())
    }
}



