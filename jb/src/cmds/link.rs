use clap::{arg, value_parser, Command};
use colored::Colorize;
use jb_lib::{tool::{Kind, Tool, Version},error::{Batch,Result}};

pub(crate) fn command() -> Command {
    Command::new("link")
        .about("Link a JetBrains tool to the PATH")
        .arg(
            arg!(tool: <TOOL> "The tool to link")
                .required(true)
                .value_parser(value_parser!(Kind)),
        )
        .arg(
            arg!(version: <VERSION>)
                .help("The release version to link (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(Version))
                .required(true),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to link the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kind = args
        .get_one::<Kind>("tool")
        .expect("Could not find argument tool");
    let version = args
        .get_one::<Version>("version")
        .expect("Could not find argument version");

    let tool = Tool::new(*tool_kind).with_version(*version);

    match tool.link() {
        Ok(()) => {}
        Err(err) => {
            return Err(Batch::from(
                err.context("Could not link tool")
            ));
        }
    }

    tracing::info!(
        "Linked {} to {}",
        tool.kind.as_str().bright_green(),
        tool.as_path().display().to_string().bright_green()
    );

    Ok(())
}