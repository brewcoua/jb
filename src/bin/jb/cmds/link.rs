use clap::{arg, value_parser, Command};
use jb::{Tool, Result};
use jb::env::Variable;
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
        Err(err) => jb::bail_with!(err, "Failed to fill {tool}"),
    };

    match tool.link() {
        Ok(()) => {}
        Err(err) => jb::bail_with!(err, "Failed to link {tool}")
    }

    jb::info!("Linked {} to {tool}", tool.kind.as_str());

    if Variable::Notify.get_bool() {
        jb::catch!(
            jb::notify(
                &format!("Linked {} to {tool}", tool.kind.as_str()),
                tool.as_icon().to_str().unwrap(),
            )
        );
    }

    Ok(())
}
