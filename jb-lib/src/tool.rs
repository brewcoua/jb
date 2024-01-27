//! `JetBrains` tool management
//!
//! This module contains functions and structs for managing `JetBrains` tools.

pub mod kind;
pub mod release;

use anyhow::{anyhow, bail, Context, Result};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::path::PathBuf;
use std::str::FromStr;

use super::util::parse::{Download, Release};
use super::util::{file, sys};

pub use kind::Kind;
pub use release::{Type, Version};

/// ## A JetBrains tool
///
/// This struct represents a JetBrains tool, such as IntelliJ IDEA or PyCharm.
/// It contains information about the tool, such as its name, version, and installation directory.
/// However, it may not be installed yet.
///
/// Every field is optional, except for `kind`. However, they are all read-only and may only be set through the builder methods.
///
/// # Examples
/// ```
/// use jb_lib::tool::{Tool, Kind};
///
/// let tool = Tool::new(Kind::RustRover)
///   .with_version("2021.2.1".parse().unwrap())
///   .with_directory("/home/user/.local/share/JetBrains".into());
/// ```
#[derive(Debug, Clone, PartialEq)]
#[readonly::make]
pub struct Tool {
    /// The kind of tool
    pub kind: Kind,
    /// The version of the tool
    pub version: Option<Version>,
    /// The directory the tool is installed to or will be installed to
    pub directory: Option<PathBuf>,
}

impl Tool {
    #[must_use]
    pub fn new(kind: Kind) -> Self {
        Self {
            kind,
            version: None,
            directory: None,
        }
    }

    /// The default installation directory for `JetBrains` tools.
    ///
    /// This may be overridden by setting the `JB_TOOLS_DIR` environment variable
    /// The default is `$HOME/.local/share/JetBrains`
    ///
    /// # Panics
    /// This function may panic if it fails to get the `HOME` environment variable.
    #[must_use]
    pub fn default_directory() -> PathBuf {
        PathBuf::from(std::env::var("HOME").unwrap()).join(".local/share/JetBrains")
    }

    /// List all installed `JetBrains` tools.
    ///
    /// This function returns a list of all installed `JetBrains` tools.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::Tool;
    /// let tools = Tool::list(None).unwrap();
    /// ```
    ///
    /// # Errors
    /// This function may return an error if it fails to read the installation directory, or any of the installed tools.
    ///
    /// # Panics
    /// This function may panic if it fails to strip the tool name prefix from the tool directory or convert the path to a string.
    pub fn list(directory: Option<PathBuf>) -> Result<Vec<Tool>> {
        let directory = directory.unwrap_or(Self::default_directory());

        let tools = kind::Kind::list();
        let mut installed_tools: Vec<Tool> = Vec::new();

        for tool in tools {
            let apps_dir = directory.join("apps");
            let tool_dirs = std::fs::read_dir(&apps_dir)
                .with_context(|| format!("Failed to read apps directory {}", apps_dir.display()))?
                .filter_map(|entry| {
                    let entry = entry.expect("Failed to read entry");
                    let path = entry.path();

                    if path.is_dir() {
                        let name = path.file_name()?.to_str()?;
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
                let tool_version = Version::from_str(
                    tool_dir
                        .strip_prefix(format!("{}-", tool.as_str()).as_str())
                        .with_context(|| {
                            format!("Failed to strip tool name prefix from {tool_dir:?}")
                        })?,
                );

                if tool_version.is_err() {
                    continue;
                }

                installed_tools.push(
                    Tool::new(*tool)
                        .with_version(tool_version.unwrap())
                        .with_directory(directory.clone()),
                );
            }
        }

        Ok(installed_tools)
    }

    /// Set the version of the tool.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///  .with_version("2021.2.1".parse().unwrap());
    /// ```
    #[must_use]
    pub fn with_version(mut self, version: Version) -> Self {
        self.version = Some(version);
        self
    }

    /// Set the installation directory of the tool.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    /// .with_directory("/home/user/.local/share/JetBrains".into());
    /// ```
    #[must_use]
    pub fn with_directory(mut self, directory: PathBuf) -> Self {
        self.directory = Some(directory);
        self
    }

    /// Check if the tool is linked.
    ///
    /// This function checks if the tool is the one linked to the icon directory.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///   .with_version("2021.2.1".parse().unwrap());
    ///
    /// if tool.is_linked() {
    ///    println!("Rust Rover is linked");
    /// }
    /// ```
    #[must_use]
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
        match icon_path.read_link() {
            Ok(link) => link.starts_with(tool_dir),
            Err(_) => false,
        }
    }

    /// Link the tool.
    ///
    /// This function links the tool to the icon directory and the binary directory.
    /// It will attempt to delete any existing symlinks, regardless of whether they exist or not (to avoid errors when linking).
    ///
    /// # Errors
    /// This function may return an error if the tool is already linked.
    /// It may also return an error if it fails to create the icon or binary symlink.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///  .with_version("2021.2.1".parse().unwrap());
    ///
    /// match tool.link() {
    ///    Ok(_) => println!("Linked Rust Rover"),
    ///   Err(e) => println!("Failed to link Rust Rover: {}", e),
    /// }
    /// ```
    pub fn link(&self) -> Result<()> {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");
        let tool_dir = self.as_path();

        if self.is_linked() {
            bail!("{} is already linked", self.kind.as_str());
        }

        // Create an icon symlink
        let icon_path = icons_dir.join(self.kind.as_str());
        let src_path = tool_dir.join(format!("bin/{}.svg", self.kind.src_name()));

        log::debug!("Linking {} to {}", self.kind.as_str(), src_path.display());

        // Delete the icon symlink, regardless of whether it exists or not
        std::fs::remove_file(&icon_path).ok();

        std::os::unix::fs::symlink(&src_path, &icon_path).with_context(|| {
            format!(
                "Failed to link {} to {}",
                self.kind.as_str(),
                src_path.display()
            )
        })?;
        log::debug!("Linked {} to {}", self.kind.as_str(), src_path.display());

        // Create a symlink to the latest version
        let binary_folder = sys::get_binary_dir()?;
        let binary_path = binary_folder.join(self.kind.as_str());

        let src_path = tool_dir.join(format!("bin/{}.sh", self.kind.src_name()));

        log::debug!("Linking {} to {}", self.kind.as_str(), src_path.display());

        // Delete the binary symlink, regardless of whether it exists or not
        std::fs::remove_file(&binary_path).ok();

        std::os::unix::fs::symlink(&src_path, &binary_path).with_context(|| {
            format!(
                "Failed to link {} to {}",
                self.kind.as_str(),
                src_path.display()
            )
        })?;
        log::debug!("Linked {} to {}", self.kind.as_str(), src_path.display());

        Ok(())
    }

    /// Unlink the tool.
    ///
    /// This function unlinks the tool from the icon directory and the binary directory.
    /// It will attempt to delete any existing symlinks, regardless of whether they exist or not (to avoid errors when unlinking).
    ///
    /// However, it will try to find an alternative version to link to, if one exists.
    ///
    /// # Errors
    /// This function may return an error if the tool is not linked.
    /// It may also return an error if it fails to remove the icon or binary symlink, as well as if it fails to create the link to the alternative version (if any).
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///   .with_version("2021.2.1".parse().unwrap());
    ///
    /// match tool.unlink() {
    ///   Ok(_) => println!("Unlinked Rust Rover"),
    ///  Err(e) => println!("Failed to unlink Rust Rover: {}", e),
    /// }
    pub fn unlink(&self) -> Result<()> {
        if !self.is_linked() {
            bail!("{} is not linked", self.kind.as_str());
        }

        let directory = self.directory.clone().unwrap_or(Self::default_directory());

        // Try to find an alternative version to link
        let mut installed_tools = Self::list(self.directory.clone())?
            .into_iter()
            .filter(|installed_tool| {
                installed_tool.kind == self.kind && installed_tool.version != self.version
            })
            .collect::<Vec<_>>();

        installed_tools.sort_by_key(|installed_tool| installed_tool.version);

        let icons_dir = directory.join("icons");

        let icon_path = icons_dir.join(self.kind.as_str());
        let binary_path = sys::get_binary_dir()?.join(self.kind.as_str());

        // Delete the icon symlink
        std::fs::remove_file(&icon_path)
            .with_context(|| format!("Failed to remove icon symlink {}", icon_path.display()))?;

        // Delete the binary symlink
        std::fs::remove_file(&binary_path).with_context(|| {
            format!("Failed to remove binary symlink {}", binary_path.display())
        })?;

        if installed_tools.is_empty() {
            log::debug!("No fallback version found for {}", self.kind.as_str());
        } else {
            // Link to the next version
            let fallback_tool = installed_tools.last().ok_or(anyhow!(
                "Failed to get fallback version for {}",
                self.kind.as_str()
            ))?;
            let fallback_path = fallback_tool.as_path();

            log::debug!(
                "Linking {} to {}",
                self.kind.as_str(),
                fallback_path.display()
            );

            let src_path = fallback_path.join(format!("bin/{}.svg", self.kind.src_name()));
            std::os::unix::fs::symlink(&src_path, &icon_path).with_context(|| {
                format!(
                    "Failed to link {} to {}",
                    self.kind.as_str(),
                    src_path.display()
                )
            })?;

            let src_path = fallback_path.join(format!("bin/{}.sh", self.kind.src_name()));
            std::os::unix::fs::symlink(&src_path, &binary_path).with_context(|| {
                format!(
                    "Failed to link {} to {}",
                    self.kind.as_str(),
                    src_path.display()
                )
            })?;
        }

        Ok(())
    }

    /// Get the name of the tool.
    ///
    /// This function returns the name of the tool, including its version.
    /// This is used for the name of the tool's directory.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///  .with_version("2021.2.1".parse().unwrap());
    ///
    /// println!("Tool name: {}", tool.name());
    /// ```
    #[must_use]
    pub fn name(&self) -> String {
        format!(
            "{}-{}",
            self.kind.as_str(),
            self.version.unwrap_or_default()
        )
    }

    /// Get the path to the tool.
    ///
    /// This function returns the path to the tool's directory.
    ///
    /// # Examples
    /// ```rust
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover)
    ///   .with_version("2021.2.1".parse().unwrap());
    ///
    /// println!("Tool path: {}", tool.as_path().display());
    /// ```
    #[must_use]
    pub fn as_path(&self) -> PathBuf {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let apps_dir = directory.join("apps");
        apps_dir.join(self.name())
    }

    /// Get the download link for the tool.
    ///
    /// This function returns the download link for the tool, including the one for the checksum and the size of the download.
    /// If no version is specified, it will return the latest version.
    ///
    /// # Errors
    /// This function may return an error if it fails to fetch the releases for the tool, or if it fails to parse the releases.
    /// It may also return an error if it fails to find a compatible download with the given version.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use jb_lib::tool::{Tool, Kind};
    /// let tool = Tool::new(Kind::RustRover);
    ///
    /// println!("Download link: {}", tool.download_link().unwrap().link);
    /// ```
    pub fn download_link(&self) -> Result<Download> {
        let latest = match self.version {
            Some(version) => version.is_latest(),
            None => true,
        };
        let release_type = match self.version {
            Some(version) => version.release,
            None => self.kind.default_type(),
        };

        let url = format!(
            "https://data.services.jetbrains.com/products/releases?code={}&latest={}&type={}",
            self.kind.as_code(),
            latest,
            release_type.as_str()
        );

        let releases_by_code = reqwest::blocking::get(&url)
            .with_context(|| {
                format!(
                    "Failed to fetch releases for {}, with URL {}",
                    self.kind.as_str(),
                    &url
                )
            })?
            .json::<HashMap<String, Vec<Release>>>()
            .with_context(|| {
                format!(
                    "Failed to parse releases for {}, with URL {}",
                    self.kind.as_str(),
                    &url
                )
            })?;

        let releases = releases_by_code
            .get(&self.kind.as_code().to_string())
            .ok_or(anyhow!(
                "Failed to find releases for {}",
                self.kind.as_str()
            ))?;

        if latest {
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
                .filter(|release| {
                    release
                        .version
                        .compare_builds(&self.version.unwrap_or_default())
                        .unwrap_or(Ordering::Equal)
                        == Ordering::Equal
                })
                .find(|release| release.rtype == self.version.unwrap_or_default().release)
                .ok_or(anyhow!("Failed to find compatible download"))?
                .download()?
                .clone())
        }
    }

    /// Install the tool.
    ///
    /// This function installs the tool to the given directory.
    /// If no directory is specified, it will install to the default directory.
    ///
    /// This function takes a mutable reference to the tool, as it will update the version of the tool to the one that was installed.
    ///
    /// # Errors
    /// This function may return an error if it fails to create the tool directory, or if the tool is already installed.
    /// It may also return an error if it fails to download the tool, or if it fails to extract the archive, as well as if it fails to link the tool.
    ///
    /// # Panics
    /// This function may panic if it fails to get the filename from the download link.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use jb_lib::tool::{Tool, Kind};
    /// let mut tool = Tool::new(Kind::RustRover)
    ///  .with_version("2021.2.1".parse().unwrap());
    ///
    /// match tool.install() {
    ///    Ok(_) => println!("Installed Rust Rover"),
    ///   Err(e) => println!("Failed to install Rust Rover: {}", e),
    /// }
    /// ```
    pub fn install(&mut self) -> Result<()> {
        let directory = self.directory.clone().unwrap_or(Self::default_directory());
        let icons_dir = directory.join("icons");

        log::debug!(
            "Fetching release '{}' for {}",
            self.version.unwrap_or_default(),
            self.kind.as_str()
        );
        let download = self.download_link()?;
        log::debug!(
            "Found download for {} with version {}",
            self.kind.as_str(),
            download.version
        );

        self.version = Some(download.version);

        let tool_dir = self.as_path();

        if tool_dir.exists() {
            bail!(
                "{} is already installed to its latest version ({})",
                self.kind.as_str(),
                download.version
            );
        }

        log::debug!("Installing {} to {}", self.name(), tool_dir.display());

        std::fs::create_dir_all(&tool_dir)
            .with_context(|| format!("Failed to create tool directory {}", tool_dir.display()))?;
        std::fs::create_dir_all(&icons_dir)
            .with_context(|| format!("Failed to create icons directory {}", icons_dir.display()))?;

        let archive_name = download
            .link
            .split('/')
            .last()
            .expect("Failed to get download filename");

        let temp_folder = sys::mktemp_dir()?;

        log::debug!("Created temporary directory {}", temp_folder.display());

        let download_path = temp_folder.join(archive_name);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        rt.block_on(file::download(
            &download.link,
            &download_path,
            download.size,
        ))?;

        log::debug!("Extracting archive to {}", tool_dir.display());

        file::extract_archive(&download_path, &tool_dir, 1)?;

        log::debug!(
            "Extracted {} to {}",
            download_path.display(),
            tool_dir.display()
        );

        // Clean up the temporary directory
        std::fs::remove_dir_all(&temp_folder).with_context(|| {
            format!(
                "Failed to remove temporary directory {}",
                temp_folder.display()
            )
        })?;
        log::debug!("Removed temporary directory {}", temp_folder.display());

        // Symlink the tool
        self.link()?;
        log::debug!("Symlinked {} to {}", self.name(), tool_dir.display());

        Ok(())
    }

    /// Uninstall the tool.
    ///
    /// This function uninstalls the tool from the given directory.
    /// If no directory is specified, it will uninstall from the default directory.
    ///
    /// # Errors
    /// This function may return an error if it fails to remove the tool directory, or if the tool is not installed.
    /// It may also return an error if it fails to unlink the tool.
    ///
    /// # Examples
    /// ```rust,no_run
    /// use jb_lib::tool::{Tool, Kind};
    ///
    /// let tool = Tool::new(Kind::RustRover)
    ///   .with_version("2021.2.1".parse().unwrap());
    ///
    /// match tool.uninstall() {
    ///    Ok(_) => println!("Uninstalled Rust Rover"),
    ///   Err(e) => println!("Failed to uninstall Rust Rover: {}", e),
    /// }
    /// ```
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
