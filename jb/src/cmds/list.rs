use clap::{arg, value_parser, Command};
use colored::Colorize;
use jb_lib::{tool::{Tool, Version}, error::{Batch, Result}};

pub(crate) fn command() -> Command {
    Command::new("list")
        .about("List installed JetBrains tools")
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to list tools from")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let directory = args.get_one::<std::path::PathBuf>("directory");

    let installed_tools = match Tool::list(directory.cloned()) {
        Ok(tools) => tools,
        Err(err) => {
            return Err(Batch::from(
                err
                    .context("Could not list installed tools")
            ));
        }
    };

    println!(
        "{:<1} {:<20} {:<20} {:<20}",
        " ",
        "Tool".bold().underline(),
        "Version".bold().underline(),
        "Release Type".bold().underline(),
    );

    let checkmark = char::from_u32(0x2714).unwrap().to_string();
    let cross = char::from_u32(0x2718).unwrap().to_string();

    for tool in installed_tools {
        let version = tool.version.unwrap_or(Version::default());

        if tool.is_linked() {
            println!(
                "{:<1} {:<20} {:<20} {:<20}",
                checkmark.green(),
                tool.kind.pretty(),
                version.to_string(),
                version.release.pretty(),
            );
        } else {
            println!(
                "{}",
                format!(
                    "{:<1} {:<20} {:<20} {:<20}",
                    cross.red(),
                    tool.kind.pretty(),
                    version.to_string(),
                    version.release.pretty(),
                )
                    .dimmed()
            );
        }
    }

    Ok(())
}