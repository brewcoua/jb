use std::path::PathBuf;
use std::str::FromStr;
use super::release::ReleaseVersion;
use super::Tool;

#[derive(Debug, Copy, Clone)]
pub struct InstalledTool {
    pub tool: Tool,
    pub version: ReleaseVersion,
}

impl InstalledTool {
    pub fn as_path(&self) -> PathBuf {
        PathBuf::from(format!("{}-{}", self.tool.as_str(), self.version))
    }
}

pub fn list_tools(directory: PathBuf) -> Vec<InstalledTool> {
    let tools = Tool::list();

    let mut installed_tools: Vec<InstalledTool> = Vec::new();

    for tool in tools {
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
            let tool_version = ReleaseVersion::from_str(
                tool_dir
                    .strip_prefix(format!("{}-", tool.as_str()).as_str())
                    .expect("Failed to strip tool name prefix")
            );

            if tool_version.is_err() {
                continue;
            }

            let tool_version = tool_version.unwrap();

            installed_tools.push(InstalledTool {
                tool,
                version: tool_version,
            });
        }
    }

    installed_tools
}