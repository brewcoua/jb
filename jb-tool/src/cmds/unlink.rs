use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, Kind, ReleaseVersion};
use anyhow::Result;

pub(crate) fn command() -> Command {
    Command::new("unlink")
        .about("Unlink a JetBrains tool from the PATH")
        .arg(
            arg!(tool: <TOOL> "The tool to unlink")
                .required(true)
                .value_parser(value_parser!(Kind))
        )
        .arg(
            arg!(version: <VERSION>)
                .help("The release version to unlink (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(ReleaseVersion))
                .required(true)
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to unlink the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false)
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kind = args.get_one::<Kind>("tool")
        .expect("Could not find argument tool")
        .clone();
    let version = args.get_one::<ReleaseVersion>("version")
        .expect("Could not find argument version")
        .clone();

    let tool = Tool::new(tool_kind)
        .with_version(version);

    tool.unlink()?;

    log::info!("Unlinked {} to {}", tool.kind().as_str(), tool.as_path().display());

    Ok(())
}