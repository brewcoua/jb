use clap::{arg, value_parser, Command};
use jb::{Tool, Result, bail};
use jb::tool::Link;

pub(crate) fn command() -> Command {
    Command::new("link")
        .about("Link a JetBrains tool to the PATH")
        .arg(
            arg!(tool: <TOOL> "The tool to link")
                .required(true)
                .value_parser(value_parser!(Tool)),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool = args
        .get_one::<Tool>("tool")
        .expect("Could not find argument tool");

    let tool = match tool.fill() {
        Ok(tool) => tool,
        Err(err) => bail!("Could not fill tool: {err}"),
    };

    match tool.link() {
        Ok(()) => {}
        Err(err) => bail!("Could not link {tool}: {err}"),
    }

    tracing::info!("Linked {} to {tool}", tool.kind.as_str());

    Ok(())
}
