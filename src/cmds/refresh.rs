use clap::{arg, value_parser, Command};
use jb_tool::tools::{Tool, release::ReleaseType, install::ToolInstaller};

pub(crate) fn command() -> Command {
    Command::new("refresh")
        .about("Update a tool to the latest version or install it if it is not installed")
        .arg(
            arg!(tool: <TOOL> "The tool to install")
                .required(true)
                .value_parser(value_parser!(Tool))
        )
        .arg(
            arg!(--type <TYPE>)
                .help("The release type to install (e.g. release, eap, preview)")
                .value_parser(value_parser!(ReleaseType))
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to install the tool to")
                .value_parser(value_parser!(std::path::PathBuf))
        )
        .arg(
            arg!(--noclean)
                .help("Do not clean up old versions of the tool")
                .required(false)
        )
}
