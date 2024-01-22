use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, Kind, ReleaseVersion};

pub(crate) fn command() -> Command {
    Command::new("link")
        .about("Symbolically link a specific version of a tool")
        .arg(
            arg!(tool: <TOOL> "The tool to link")
                .required(true)
                .value_parser(value_parser!(Kind))
        )
        .arg(
            arg!(version: <VERSION>)
                .help("The release version to link (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(ReleaseVersion))
                .required(true)
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to install the tool to")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false)
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) {
    let tool_kind = args.get_one::<Kind>("tool")
        .expect("Could not find argument tool")
        .clone();
    let version = args.get_one::<ReleaseVersion>("version")
        .expect("Could not find argument version")
        .clone();

    let tool = Tool::new(tool_kind)
        .with_version(version);

    let result = tool.link();

    match result {
        Ok(_) => {
            log::info!("Linked {} to {}", tool.kind().as_str(), tool.as_path().display());
        },
        Err(e) => {
            log::error!("Failed to link tool:\n{}", e);
            std::process::exit(1);
        }
    }
}