//! List tools
//!
//! This module provides the ability to list installed tools.

use std::path::PathBuf;
use std::str::FromStr;

use crate::env::Variable;
use crate::tool::{Tool, kind::Kind};

pub trait List {
    /// Lists all tools.
    ///
    /// Any tools that are not valid will be skipped. (logged as debug)
    ///
    /// # Errors
    /// This function will return an error if the tools directory does not exist or if the tools cannot be listed.
    fn list() -> anyhow::Result<Vec<Tool>> where Self: Sized;

    /// Lists tools of a specific kind.
    ///
    /// # Errors
    /// This function will return an error if the tools directory does not exist or if the tools cannot be listed.
    fn list_kind(kind: Kind) -> anyhow::Result<Vec<Tool>> where Self: Sized;

    /// Lists tools that match the current tool.
    ///
    /// # Errors
    /// This function will return an error if the tools directory does not exist or if the tools cannot be listed.
    fn list_matching(&self) -> anyhow::Result<Vec<Tool>> where Self: Sized;
}

impl List for Tool {
    fn list() -> anyhow::Result<Vec<Tool>> {
        let tools_directory = Variable::ToolsDirectory.get::<PathBuf>();

        let mut tools = vec![];

        for entry in std::fs::read_dir(tools_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name= path.file_name().unwrap().to_str().unwrap();

                if let Ok(tool) = Tool::from_str(name) {
                    tools.push(tool);
                } else {
                    crate::debug!("Skipping invalid tool directory: {name}");
                }
            }
        }

        Ok(tools)
    }

    fn list_kind(kind: Kind) -> anyhow::Result<Vec<Tool>> {
        Ok(
            Self::list()?
                .into_iter()
                .filter(|tool| tool.kind == kind)
                .collect()
        )
    }

    fn list_matching(&self) -> anyhow::Result<Vec<Tool>> {
        Ok(
            Self::list()?
                .into_iter()
                .filter(|tool| self.matched(tool))
                .collect()
        )
    }
}