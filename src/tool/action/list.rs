//! List tools
//!
//! This module provides the ability to list installed tools.

use std::path::PathBuf;
use std::str::FromStr;

use crate::env::Variable;
use crate::tool::{Tool, kind::Kind};

pub trait List {
    fn list() -> anyhow::Result<Vec<Tool>> where Self: Sized;
    fn list_kind(kind: Kind) -> anyhow::Result<Vec<Tool>> where Self: Sized;
}

impl List for Tool {
    fn list() -> anyhow::Result<Vec<Tool>> {
        let span = tracing::debug_span!("list");
        let _enter = span.enter();

        let tools_directory = Variable::ToolsDirectory.get::<PathBuf>();

        let mut tools = vec![];

        for entry in std::fs::read_dir(tools_directory)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let name= path.file_name().unwrap().to_str().unwrap();

                match Tool::from_str(name) {
                    Ok(tool) => tools.push(tool),
                    Err(_) => {
                        tracing::debug!("Skipping invalid tool directory: {}", name);
                        continue;
                    }
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
}