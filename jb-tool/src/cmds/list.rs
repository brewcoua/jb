use colored::Colorize;
use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, ReleaseVersion};
use anyhow::Result;

pub(crate) fn command() -> Command {
    Command::new("list")
        .about("List all installed tools and their versions")
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to list tools from")
                .value_parser(value_parser!(std::path::PathBuf))
        )
}



pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let directory = args.get_one::<std::path::PathBuf>("directory");

    let installed_tools = Tool::list(directory)?;

    println!(
        "{:<20} {:<20} {:<20}",
        "Tool".bold().underline(),
        "Version".bold().underline(),
        "Release Type".bold().underline(),
    );

    for tool in installed_tools {
        let version = tool.version().unwrap_or(ReleaseVersion::default());

        if tool.is_linked() {
            println!(
                "{:<20} {:<20} {:<20}",
                tool.kind().pretty(),
                version.to_string(),
                version.release.pretty(),
            );
        } else {
            println!(
                "{}",
                format!(
                    "{:<20} {:<20} {:<20}",
                    tool.kind().pretty(),
                    version.to_string(),
                    version.release.pretty(),
                ).dimmed()
            );
        }
    }

    Ok(())
}