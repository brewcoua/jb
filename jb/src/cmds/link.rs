use clap::{arg, value_parser, Command};
use jb_lib::{tool::Tool,error::{Batch,Result}};

pub(crate) fn command() -> Command {
    Command::new("link")
        .about("Link a JetBrains tool to the PATH")
        .arg(
            arg!(tool: <TOOL> "The tool to link")
                .required(true)
                .value_parser(value_parser!(Tool)),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to link the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool = args
        .get_one::<Tool>("tool")
        .expect("Could not find argument tool");

    match tool.link() {
        Ok(()) => {}
        Err(err) => {
            return Err(Batch::from(
                err.context("Could not link tool")
            ));
        }
    }

    tracing::info!("Linked {} to {tool}", tool.kind.as_str());

    Ok(())
}
