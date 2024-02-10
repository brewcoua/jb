use clap::{Command};
use jb::Batch;
use jb::env::Variable;
use jb::tool::{Tool, List, Kind};
use crate::emoji::{CHECK, DESKTOP};

pub(crate) fn command() -> Command {
    Command::new("desktop")
        .about("Create or update the desktop entries for installed tools")
        .long_about("Create or update the desktop entries for installed tools. It will create the desktop entry regardless of whether the tool is linked or not.\nHowever, all paths in the desktop entry are only relative to the linked tool.")
}

pub(crate) fn dispatch() -> jb::Result<()> {
    let mut installed_tools = jb::catch!(Tool::list());
    installed_tools.sort_by(|a, b| a.kind.cmp(&b.kind));
    installed_tools.dedup_by(|a, b| a.kind == b.kind);

    let kinds = installed_tools.iter().map(|tool| tool.kind).collect::<Vec<_>>();

    if kinds.is_empty() {
        jb::bail!("No tools installed, nothing to do");
    }

    jb::info!("{DESKTOP} Creating desktop entries for installed tools...");

    let mut error_batch = Batch::new();

    let desktop_path = Variable::DesktopDirectory.get::<std::path::PathBuf>();

    // Delete all existing desktop entries
    for kind in Kind::list() {
        let path = desktop_path.join(format!("jetbrains_{}.desktop", kind.as_str()));
        if path.exists() {
            jb::debug!("Deleting existing desktop entry at {}", path.display());
            jb::catch_with!(error_batch, std::fs::remove_file(&path));
        }
    }

    for kind in kinds {
        let tool = Tool::from_kind(kind);
        let desktop = tool.as_desktop();

        let path = desktop_path.join(format!("jetbrains_{}.desktop", kind.as_str()));

        jb::debug!("Writing desktop entry to {}", path.display());

        jb::catch_with!(error_batch, std::fs::write(&path, desktop));
    }

    if error_batch.is_empty() {
        jb::info!("{CHECK} Desktop entries created successfully");
        Ok(())
    } else {
        jb::error!("Failed to create desktop entries");
        Err(error_batch)
    }
}