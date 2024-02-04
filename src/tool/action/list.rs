//! List tools
//!
//! This module provides the ability to list installed tools.

use std::path::PathBuf;
use std::str::FromStr;

use crate::env::Variable;
use crate::tool::{Tool, kind::Kind};

pub trait List {
    fn list() -> anyhow::Result<Vec<Tool>>;
    fn list_kind(kind: Kind) -> anyhow::Result<Vec<Tool>>;
}

impl List for Tool {
    fn list() -> anyhow::Result<Vec<Tool>> {
        let directory = Variable::get::<PathBuf>(Variable::ToolsDirectory)?;
        let tools_directory = directory.join("apps");

        let mut tools = vec![];

        for entry in std::fs::read_dir(tools_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name= path.file_name().unwrap().to_str().unwrap();
                let tool = Tool::from_str(name)?
                    .with_context(|| format!("Failed to parse tool: {}", name))?;
                tools.push(tool);
            }
        }

        Ok(tools)
    }

    fn list_kind(kind: Kind) -> anyhow::Result<Vec<Tool>> {
        Self::list()?.into_iter()
            .filter(|tool| tool.kind == kind)
            .collect()
    }
}