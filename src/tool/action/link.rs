//! Link tools
//!
//! This module provides the ability to link and unlink tools, which is useful for setting up the PATH environment variable.

use std::path::{Path, PathBuf};
use anyhow::Context;
use super::list::List;
use crate::env::Variable;
use crate::tool::Tool;

pub trait Link {
    fn is_linked(&self) -> bool where Self: Sized;
    fn link(&self) -> anyhow::Result<()> where Self: Sized;
    fn unlink(&self) -> anyhow::Result<()> where Self: Sized;
}

impl Link for Tool {
    fn is_linked(&self) -> bool {
        let span = tracing::debug_span!("is_linked", tool = self.as_str());
        let _enter = span.enter();

        let tool_directory = self.as_path();

        let binary_path = tool_directory.join(format!("bin/{}.sh", self.kind.binary()));
        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();


        // Check if linked binary is the right one (not any other version or simply doesn't exist)
        let binary = binaries_directory.join(self.kind.as_str());
        if !binary.exists() || std::fs::read_link(&binary).ok() != Some(binary_path.clone()) {
            tracing::debug!("Binary is not linked: {}, {:?} != {:?}", binary.exists(), std::fs::read_link(binary).ok(), Some(binary_path));
            return false;
        }

        tracing::debug!("Binary is linked");


        let icon_path = tool_directory.join(format!("bin/{}.svg", self.kind.binary()));
        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        // Check if linked icon is the right one (not any other version or simply doesn't exist)
        let icon = icons_directory.join(self.kind.as_str());
        if !icon.exists() || std::fs::read_link(icon).ok() != Some(icon_path) {
            tracing::debug!("Icon is not linked");
            return false;
        }

        tracing::debug!("Icon is linked");

        true
    }

    fn link(&self) -> anyhow::Result<()> {
        let span = tracing::debug_span!("link", tool = self.as_str());
        let _enter = span.enter();

        if self.is_linked() {
            tracing::warn!("{} is already linked", self.as_str());
            return Ok(());
        }

        let tool_directory = self.as_path();

        let binary_path = tool_directory.join(format!("bin/{}.sh", self.kind.binary()));
        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();

        if !binaries_directory.exists() {
            std::fs::create_dir_all(&binaries_directory)?;
        }

        symlink(&binary_path, binaries_directory.join(self.kind.as_str()))?;

        tracing::debug!("Linked binary");

        let icon_path = tool_directory.join(format!("bin/{}.svg", self.kind.binary()));

        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        if !icons_directory.exists() {
            std::fs::create_dir_all(&icons_directory)?;
        }

        symlink(&icon_path, icons_directory.join(self.kind.as_str()))?;

        tracing::debug!("Linked icon");

        Ok(())
    }

    fn unlink(&self) -> anyhow::Result<()> {
        let span = tracing::info_span!("unlink", tool = self.as_str());
        let _enter = span.enter();

        if !self.is_linked() {
            tracing::warn!("{} is not linked", self.as_str());
            return Ok(());
        }

        let binaries_directory = Variable::BinariesDirectory.get::<PathBuf>();

        std::fs::remove_file(binaries_directory.join(self.kind.as_str()))?;

        tracing::debug!("Unlinked binary");

        let icons_directory = Variable::IconsDirectory.get::<PathBuf>();

        std::fs::remove_file(icons_directory.join(self.kind.as_str()))?;

        tracing::debug!("Unlinked icon");

        // Find an alternative version to link
        let mut tools = Tool::list_kind(self.kind)
            .with_context(|| format!("Failed to list installed tools for {}", self.kind))?;
        tools.sort();

        if let Some(tool) = tools.first() {
            tracing::debug!("Found alternative version: {}", tool.as_str());
            tool.link()?;
            tracing::debug!("Linked alternative version");
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