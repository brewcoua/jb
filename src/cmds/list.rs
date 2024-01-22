use colored::Colorize;
use clap::{arg, value_parser, Command};
use jb_tool::tools::list;

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
    let directory = args.get_one::<std::path::PathBuf>("directory").unwrap_or(
        &std::path::PathBuf::from(
            std::env::var("HOME")
                .expect("Failed to get home directory, please use -d/--directory to specify a directory")
        ).join(".local/share/JetBrains")
    ).clone();

    let installed_tools = list::list_tools(directory);

    println!(
        "{:<20} {:<20} {:<20}",
        "Tool".bold().underline(),
        "Version".bold().underline(),
        "Release Type".bold().underline(),
    );

    for tool in installed_tools {
        println!(
            "{:<20} {:<20} {:<20}",
            tool.tool.pretty(),
            tool.version.to_string(),
            tool.version.release.pretty()
        );
    }
}