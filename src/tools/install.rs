use std::path::PathBuf;
use super::{Tool, release:: ReleaseType };
use crate::{ utils::sys, utils::file };

pub struct ToolInstaller {
    tool: Tool,
    release_type: ReleaseType,
    directory: PathBuf,
}

impl ToolInstaller {
    pub fn new(tool: Tool, release_type: Option<&ReleaseType>, directory: PathBuf) -> Self {
        let release_type = release_type.unwrap_or(&tool.default_type()).clone();

        Self {
            tool,
            release_type,
            directory,
        }
    }

    pub fn install(self) -> Result<bool, &'static str> {
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
}