use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, Kind, ReleaseVersion};
use anyhow::Result;
use colored::Colorize;

pub(crate) fn command() -> Command {
    Command::new("install")
        .about("Install a JetBrains tool")
        .arg(
            arg!(tool: <TOOL> "The tool to install")
                .required(true)
                .value_parser(value_parser!(Kind))
        )
        .arg(
            arg!(--build <VERSION>)
                .help("The release version to install (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(ReleaseVersion))
                .required(false)
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to install the tool to")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false)
        )
        .arg(
            arg!(--clean)
                .help("Clean up old versions after installing")
                .required(false)
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kind: &Kind = args.get_one::<Kind>("tool").expect("Could not find argument tool");
    let version: Option<&ReleaseVersion> = args.get_one::<ReleaseVersion>("build");
    let directory: Option<&std::path::PathBuf> = args.get_one::<std::path::PathBuf>("directory");

    // Get all folders matching the tool name in the given directory (directory/apps/tool-*)

    let mut tool = Tool::new(tool_kind.clone());
    if !version.is_none() {
        tool = tool.with_version(version.unwrap().clone());
    }
    if !directory.is_none() {
        tool = tool.with_directory(directory.unwrap().clone());
    }

    tool.install()?;

    log::info!("Installed {} to {}", tool.kind().as_str().bright_green(), tool.as_path().display().to_string().bright_green());

    if args.get_flag("clean") {
        // Clean up old versions
        log::info!("Cleaning up old versions of {}", tool.kind().as_str().bright_green());

        let installed_tools = Tool::list(directory)?
            .into_iter()
            .filter(|t| t.kind() == tool.kind() && t.version() != tool.version())
            .collect::<Vec<Tool>>();

        for tool in installed_tools {
            tool.uninstall()?;
            log::info!("Uninstalled {}", tool.as_path().display().to_string().bright_green());
        }

        log::info!("Cleaned up old versions of {}", tool.kind().as_str().bright_green());
    }

    Ok(())
}
