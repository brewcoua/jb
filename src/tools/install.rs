use std::path::PathBuf;
use super::{Kind, release::{ReleaseType, ReleaseVersion}, list::InstalledTool};
use crate::{ utils::sys, utils::file };

pub struct ToolInstaller {
    tool: Kind,
    release_type: ReleaseType,
    directory: PathBuf,
    version: Option<ReleaseVersion>
}

impl ToolInstaller {
    pub fn new(tool: Kind, release_type: Option<&ReleaseType>, directory: PathBuf) -> Self {
        let release_type = release_type.unwrap_or(&tool.default_type()).clone();

        Self {
            tool,
            release_type,
            directory,
            version: None,
        }
    }

    pub fn with_version(mut self, version: ReleaseVersion) -> Self {
        self.version = Some(version);
        self
    }

    pub fn install(&self) -> Result<bool, &'static str> {
        log::debug!("Fetching latest release for {}", self.tool.as_str());
        let release = self.tool.latest_release(&self.release_type);

        log::info!("Found latest release {}", release.version);

        // Find the download for the current platform
        let download = release.download();
        return if let Some(download) = download {
            let apps_dir = self.directory.join("apps");
            let icons_dir = self.directory.join("icons");

            std::fs::create_dir_all(&apps_dir).expect("Failed to create apps directory");
            std::fs::create_dir_all(&icons_dir).expect("Failed to create icons directory");

            let tool_dir = apps_dir.join(format!("{}-{}", self.tool.as_str(), release.version));
            if tool_dir.exists() {
                log::info!("{} is already installed to its latest version ({})", self.tool.as_str(), release.version);
                return Ok(false);
            } else {
                std::fs::create_dir_all(&tool_dir).expect("Failed to create tool directory");
            }

            log::info!("Installing {} to {}", self.tool.as_str(), tool_dir.display());

            let archive_name = download.link.split('/').last().expect("Failed to get download filename");

            // Download the archive to a temporary folder
            let temp_folder = sys::mktemp_dir().expect("Failed to create temporary directory");

            log::debug!("Created temporary directory {}", temp_folder.display());

            let download_path = temp_folder.join(&archive_name);
            file::download_file(&download.link, &download_path, download.size)
                .expect("Failed to download file");

            log::debug!("Downloaded {} to {}", download.link, download_path.display());

            file::extract_archive(&download_path, &tool_dir, 1)
                .expect("Failed to extract archive");

            log::debug!("Extracted {} to {}", download_path.display(), tool_dir.display());

            // Clean up the temporary directory
            std::fs::remove_dir_all(&temp_folder).expect("Failed to remove temporary directory");
            log::debug!("Removed temporary directory {}", temp_folder.display());

            // Create a symlink to the latest version
            let binary_folder = sys::get_binary_dir().expect("Failed to get binary directory");
            let binary_path = binary_folder.join(self.tool.as_str());

            let src_path = tool_dir.join(format!("bin/{}.sh", self.tool.src_name()));

            sys::symlink(&src_path, &binary_path).expect("Failed to create symlink");

            // Create an icon symlink
            let icon_path = icons_dir.join(self.tool.as_str());
            let src_path = tool_dir.join(format!("bin/{}.svg", self.tool.src_name()));

            sys::symlink(&src_path, &icon_path).expect("Failed to create symlink");

            log::info!("Successfully installed {} to {}", self.tool.as_str(), tool_dir.display());

            Ok(true)
        } else {
            log::error!("Failed to find compatible download for {}", self.tool.as_str());
            Err("Failed to find compatible download")
        }
    }

    pub fn uninstall(&self) -> Result<(), &'static str> {
        if let Some(version) = self.version {
            self.uninstall_version(version)?;
        } else {
            let installed_tools = super::list::list_tools(self.directory.clone())
                .into_iter()
                .filter(|installed_tool| installed_tool.tool == self.tool);

            for installed_tool in installed_tools {
                self.uninstall_version(installed_tool.version)?;
            }
        }

        Ok(())
    }

    fn uninstall_version(&self, version: ReleaseVersion) -> Result<bool, &'static str> {
        let apps_dir = self.directory.join("apps");
        let tool_dir = apps_dir.join(format!("{}-{}", self.tool.as_str(), version));

        if tool_dir.exists() {
            std::fs::remove_dir_all(&tool_dir).expect("Failed to remove tool directory");
            log::info!("Successfully uninstalled {} from {}", self.tool.as_str(), tool_dir.display());

            // Check if tool is symlinked
            let binary_folder = sys::get_binary_dir().expect("Failed to get binary directory");
            let binary_path = binary_folder.join(self.tool.as_str());

            // Check if symlink path is tool directory
            if binary_path.exists() && binary_path.read_link().expect("Failed to read symlink") == tool_dir.join(format!("bin/{}.sh", self.tool.src_name())) {
                // Search for a fallback
                let mut installed_tools = super::list::list_tools(self.directory.clone())
                    .into_iter()
                    .filter(|installed_tool| installed_tool.tool == self.tool)
                    .filter(|installed_tool| installed_tool.version != version)
                    .collect::<Vec<_>>();

                installed_tools.sort_by_key(|installed_tool| installed_tool.version);

                if let Some(installed_tool) = installed_tools.last() {
                    let installed_tool_path = apps_dir.join(installed_tool.as_path());

                    let src_path = installed_tool_path.join(format!("bin/{}.sh", self.tool.src_name()));
                    let dest_path = binary_folder.join(self.tool.as_str());

                    sys::symlink(&src_path, &dest_path).expect("Failed to create symlink");
                    log::info!("Successfully created symlink from {} to {}", src_path.display(), dest_path.display());

                    let src_path = installed_tool_path.join(format!("bin/{}.svg", self.tool.src_name()));
                    let dest_path = self.directory.join("icons").join(self.tool.as_str());

                    sys::symlink(&src_path, &dest_path).expect("Failed to create symlink");
                    log::info!("Successfully created symlink from {} to {}", src_path.display(), dest_path.display());
                } else {
                    std::fs::remove_file(&binary_path).expect("Failed to remove symlink");
                    log::info!("Successfully removed symlink from {} to {}", binary_path.display(), tool_dir.display());

                    let dest_path = self.directory.join("icons").join(self.tool.as_str());
                    std::fs::remove_file(&dest_path).expect("Failed to remove symlink");
                    log::info!("Successfully removed symlink from {} to {}", dest_path.display(), tool_dir.display());
                }
            }

            Ok(true)
        } else {
            log::info!("{} is not installed", self.tool.as_str());
            Ok(false)
        }
    }
}