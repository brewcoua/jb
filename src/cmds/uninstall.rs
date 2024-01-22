use clap::{arg, value_parser, Command};
use jb_tool::tools::{Kind, release::ReleaseType, install::ToolInstaller, list};
use jb_tool::tools::release::ReleaseVersion;

pub(crate) fn command() -> Command {
    Command::new("uninstall")
        .about("Uninstall a tool, removing all versions or a specific version")
        .arg(
            arg!(tool: <TOOL> "The tool to install")
                .required(true)
                .value_parser(value_parser!(Kind))
        )
        .arg(
            arg!(--build <VERSION>)
                .help("The build number to install (e.g. 2021.1.1)")
                .value_parser(value_parser!(ReleaseType))
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to uninstall the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) {
    let directory = args.get_one::<std::path::PathBuf>("directory").unwrap_or(
        &std::path::PathBuf::from(
            std::env::var("HOME")
                .expect("Failed to get home directory, please use -d/--directory to specify a directory")
        ).join(".local/share/JetBrains")
    ).clone();

    let tool = args.get_one::<Kind>("tool")
        .expect("Failed to get tool")
        .clone();
    let build = args.get_one::<ReleaseVersion>("build");

    let mut installer = ToolInstaller::new(tool, None, directory);

    if !build.is_none() {
        installer = installer.with_version(build.unwrap().clone());
    }

    installer.uninstall()
        .expect("Failed to uninstall tool")
}