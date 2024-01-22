use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, Kind, ReleaseVersion};

pub(crate) fn command() -> Command {
    Command::new("install")
        .about("Install any of JetBrains' IDEs and tools")
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

pub(crate) fn dispatch(args: &clap::ArgMatches) {
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

    let result = tool.install();

    if let Err(e) = result {
        log::error!("Failed to install tool:\n{}", e);
        std::process::exit(1);
    }

    log::info!("Installed {} to {}", tool.kind().as_str(), tool.as_path().display());

    if args.get_flag("clean") {
        // TODO: Clean up old versions (to be done after uninstall method is implemented)
        todo!();
    }
}
