use clap::{arg, value_parser, Command};
use colored::Colorize;
use jb::{Tool, Result, bail_with};
use jb::tool::Link;

pub(crate) fn command() -> Command {
    Command::new("unlink")
        .about("Unlink a JetBrains tool from the PATH")
        .arg(
            arg!(tool: <TOOL> "The tool to unlink")
                .required(true)
                .value_parser(value_parser!(Tool)),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to unlink the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool = args
        .get_one::<Tool>("tool")
        .expect("Could not find argument tool");

    match tool.unlink() {
        Ok(()) => {}
        Err(err) => bail_with!(err, "Failed to unlink {}", tool.as_path().display())
    }

    tracing::info!(
        "Unlinked {} to {}",
        tool.kind.as_str().bright_green(),
        tool.as_path().display().to_string().bright_green()
    );

    Ok(())
}
