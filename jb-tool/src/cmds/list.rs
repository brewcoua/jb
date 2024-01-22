use colored::Colorize;
use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, ReleaseVersion};

pub(crate) fn command() -> Command {
    Command::new("list")
        .about("List all installed tools and their versions")
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to list tools from")
                .value_parser(value_parser!(std::path::PathBuf))
        )
}



pub(crate) fn dispatch(args: &clap::ArgMatches) {
    let directory = args.get_one::<std::path::PathBuf>("directory");

    let result = Tool::list(directory);

    if let Err(e) = result {
        log::error!("Failed to list tools:\nError: {}", e);
        std::process::exit(1);
    }

    let installed_tools = result.unwrap();

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
                "{:<20} {:<20} {:<20}",
                tool.kind().pretty().dimmed(),
                version.to_string().dimmed(),
                version.release.pretty().dimmed(),
            );
        }
    }
}