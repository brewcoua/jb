use anyhow::Context;

use super::list::List;
use super::link::Link;
use crate::{api,util};
use crate::tool::Tool;
use crate::debug;

pub trait Install {
    /// Returns whether the tool is installed.
    ///
    /// This will check if the tool's directory exists and if the tool is linked.
    ///
    /// # Errors
    /// This function will return an error if the tool's directory cannot be checked.
    fn is_installed(&self) -> anyhow::Result<bool> where Self: Sized;

    /// Installs the tool.
    ///
    /// This will download the tool's release, extract it, and link the tool.
    ///
    /// # Errors
    /// This function will return an error if the tool is already installed, or if the tool cannot be installed.
    fn install(&mut self) -> anyhow::Result<()> where Self: Sized;

    /// Uninstalls the tool.
    ///
    /// This will remove the tool's directory and unlink the tool.
    ///
    /// # Errors
    /// This function will return an error if the tool is not installed, or if the tool cannot be removed.
    fn uninstall(&self) -> anyhow::Result<()> where Self: Sized;
}

impl Install for Tool {
    fn is_installed(&self) -> anyhow::Result<bool> {
        debug!("Checking if installed");

        Ok(
            Tool::list_kind(self.kind)
                .with_context(|| format!("Failed to list installed tools for {}", self.kind))?
                .iter()
                .any(|tool| self.matched(tool))
        )
    }

    fn install(&mut self) -> anyhow::Result<()> {
        let result = api::fetch::release(self)?;

        crate::info!("Found release: {}", result.tool);

        self.version = Some(result.tool.version.unwrap());
        self.build = Some(result.tool.build.unwrap());
        self.release = Some(result.tool.release.unwrap());

        let installed = self.is_installed()?;
        if installed {
            crate::warn!("{} is already installed", self.as_str());
            return Ok(());
        }

        let tempdir = tempfile::tempdir()
            .with_context(|| "Failed to create temporary directory")?;

        let process = || {
            let archive_name = result
                .download.link
                .rsplit('/')
                .next()
                .with_context(|| "Failed to get archive name")?;

            crate::info!("Downloading {} ({})", archive_name, humansize::format_size(result.download.size, humansize::DECIMAL));

            let archive_path = tempdir.path().join(archive_name);
            util::download(&result.download.link, &archive_path, Some(result.download.size))?;

            let tool_directory = self.as_path();
            if !tool_directory.exists() {
                std::fs::create_dir_all(&tool_directory)
                    .with_context(|| format!("Failed to create {}", tool_directory.display()))?;
            }

            crate::info!("Extracting to {}", tool_directory.display());

            util::extract_archive(&archive_path, &tool_directory, 1)?;

            crate::info!("Linking {}", self.as_str());
            self.link()?;

            Ok(())
        };

        let output = process();

        // Clean up the temporary directory
        tempdir.close()
            .with_context(|| "Failed to clean up temporary directory")?;

        match output {
            Ok(()) => {
                crate::info!("Installed {}", self.as_str());
                Ok(())
            }
            Err(e) => {
                crate::error!("Failed to install {}: {e}", self.as_str());
                Err(e)
            }
        }
    }

    fn uninstall(&self) -> anyhow::Result<()> {
        let installed = self.is_installed()?;
        if !installed {
            crate::warn!("{} is not installed", self.as_str());
            return Ok(());
        }

        crate::info!("Unlinking {}", self.as_str());
        self.unlink()?;

        let tool_directory = self.as_path();
        crate::info!("Removing {}", tool_directory.display());
        std::fs::remove_dir_all(&tool_directory)
            .with_context(|| format!("Failed to remove {}", tool_directory.display()))?;

        crate::info!("Uninstalled {}", self.as_str());

        Ok(())
    }
}
