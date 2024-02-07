//! Link tools
//!
//! This module provides the ability to link and unlink tools, which is useful for setting up the PATH environment variable.

use std::path::{Path, PathBuf};
use anyhow::Context;
use super::list::List;
use crate::env::Variable;
use crate::tool::Tool;

pub trait Link {
    /// Returns whether the tool is linked.
    fn is_linked(&self) -> bool where Self: Sized;

    /// Links the tool.
    ///
    /// This creates a symlink to the tool's binary and icon in the PATH and icons directory, respectively.
    ///
    /// # Errors
    /// This function will return an error if the tool is already linked, or if the symlinks fail.
    fn link(&self) -> anyhow::Result<()> where Self: Sized;

    /// Unlinks the tool.
    ///
    /// This removes the symlink to the tool's binary and icon in the PATH and icons directory, respectively.
    ///
    /// # Errors
    /// This function will return an error if the tool is not linked, or if the symlinks fail.
    fn unlink(&self) -> anyhow::Result<()> where Self: Sized;

    /// Unlinks the tool and links an alternative version.
    ///
    /// This removes the symlink to the tool's binary and icon in the PATH and icons directory, respectively, and try to link an alternative version if available.
    ///
    /// # Errors
    /// This function will return an error if the tool is not linked or if the symlinks fail.
    fn unlink_with_alternative(&self) -> anyhow::Result<()> where Self: Sized;
}

impl Link for Tool {
    fn is_linked(&self) -> bool {
        let tool_directory = self.as_path();

        let binary_path = tool_directory.join(format!("bin/{}.sh", self.kind.binary()));
        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();


        // Check if linked binary is the right one (not any other version or simply doesn't exist)
        let binary = binaries_directory.join(self.kind.as_str());
        if !binary.exists() || std::fs::read_link(&binary).ok() != Some(binary_path.clone()) {
            crate::debug!("Binary is not linked: {}, {:?} != {:?}", binary.exists(), std::fs::read_link(binary).ok(), Some(binary_path));
            return false;
        }

        crate::debug!("Binary is linked");


        let icon_path = tool_directory.join(format!("bin/{}.svg", self.kind.binary()));
        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        // Check if linked icon is the right one (not any other version or simply doesn't exist)
        let icon = icons_directory.join(self.kind.as_str());
        if !icon.exists() || std::fs::read_link(icon).ok() != Some(icon_path) {
            crate::debug!("Icon is not linked");
            return false;
        }

        crate::debug!("Icon is linked");

        true
    }

    fn link(&self) -> anyhow::Result<()> {
        if self.is_linked() {
            crate::warn!("{} is already linked", self.as_str());
            return Ok(());
        }

        let tool_directory = self.as_path();

        let binary_path = tool_directory.join(format!("bin/{}.sh", self.kind.binary()));
        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();

        if !binaries_directory.exists() {
            std::fs::create_dir_all(&binaries_directory)?;
        }

        symlink(binary_path, binaries_directory.join(self.kind.as_str()))?;

        crate::debug!("Linked binary");

        let icon_path = tool_directory.join(format!("bin/{}.svg", self.kind.binary()));

        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        if !icons_directory.exists() {
            std::fs::create_dir_all(&icons_directory)?;
        }

        symlink(icon_path, icons_directory.join(self.kind.as_str()))?;

        crate::debug!("Linked icon");

        Ok(())
    }

    fn unlink(&self) -> anyhow::Result<()> {
        if !self.is_linked() {
            anyhow::bail!("{} is not linked", self.as_str());
        }

        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();

        std::fs::remove_file(binaries_directory.join(self.kind.as_str()))?;

        crate::debug!("Unlinked binary");

        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        std::fs::remove_file(icons_directory.join(self.kind.as_str()))?;

        crate::debug!("Unlinked icon");

        Ok(())
    }

    fn unlink_with_alternative(&self) -> anyhow::Result<()> {
        self.unlink()?;

        // Find an alternative version to link
        let mut tools = Tool::list_kind(self.kind)
            .with_context(|| format!("Failed to list installed tools for {}", self.kind))?;
        tools.retain(|tool| tool != self);
        tools.sort();

        if let Some(tool) = tools.first() {
            crate::debug!("Found alternative version: {}", tool.as_str());
            tool.link()?;
            crate::debug!("Linked alternative version {tool}");
        }

        Ok(())
    }
}

fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(src: P, dst: Q) -> anyhow::Result<()> {
    let src = src.as_ref();
    let dst = dst.as_ref();

    // Try removing the destination file anyway since it might be a broken symlink
    // Broken symlinks show up as non-existent files, and may cause the symlink to fail
    std::fs::remove_file(dst).ok();

    #[cfg(unix)]
    std::os::unix::fs::symlink(src, dst)?;

    #[cfg(windows)]
    std::os::windows::fs::symlink_file(src, dst)?;

    Ok(())
}