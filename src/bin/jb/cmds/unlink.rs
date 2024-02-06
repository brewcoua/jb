use clap::{arg, value_parser, Command};
use jb::{Tool, Result};
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

    let tool = match tool.fill() {
        Ok(tool) => tool,
        Err(err) => jb::bail_with!(err, "Failed to fill {tool}")
    };

    match tool.unlink() {
        Ok(()) => {}
        Err(err) => jb::bail_with!(err, "Failed to unlink {tool}")
    }

    jb::info!(
        "Unlinked {} from {}",
        tool.kind.as_str(),
        tool.as_path().display(),
    );

    Ok(())
}
