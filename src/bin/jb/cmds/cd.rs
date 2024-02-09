use clap::Command;
use jb::env::Variable;
use std::io::IsTerminal;
use crate::emoji::FOLDER;

pub(crate) fn command() -> Command {
    Command::new("cd")
        .about("Open a new shell in the tools directory")
}

pub(crate) fn dispatch() -> jb::Result<()> {
    let tools_dir = Variable::ToolsDirectory.get::<std::path::PathBuf>();

    if std::io::stdout().is_terminal() {
        jb::info!("{FOLDER} Opening {}", tools_dir.display());
    } else {
        jb::bail!(anyhow::anyhow!("The current process is not attached to a terminal, so the directory cannot be changed"));
    }

    // Get current shell and change directory by creating a new shell process in the tools directory
    // This is a workaround for the fact that Rust doesn't have a way to change the current directory of the current process

    if let Err(err) = std::env::set_current_dir(&tools_dir) {
        jb::bail!(err);
    }

    let shell = std::env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let status = std::process::Command::new(shell)
        .arg("-i")
        .status();

    if let Err(err) = status {
        jb::bail!(err);
    }

    Ok(())
}