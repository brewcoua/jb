mod install;
mod link;
mod list;
mod uninstall;
mod unlink;

use anyhow::{bail, Result};
use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("jb")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A tool to install and manage JetBrains' IDEs and tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            arg!(-v - -verbose)
                .help("Enable verbose logging")
                .required(false)
                .global(true),
        )
        .subcommand(install::command())
        .subcommand(uninstall::command())
        .subcommand(list::command())
        .subcommand(link::command())
        .subcommand(unlink::command())
}

pub fn dispatch(args: Option<(&str, &clap::ArgMatches)>) -> Result<()> {
    if let Some((name, sub_matches)) = args {
        match name {
            "install" => install::dispatch(sub_matches),
            "uninstall" => uninstall::dispatch(sub_matches),
            "list" => list::dispatch(sub_matches),
            "link" => link::dispatch(sub_matches),
            "unlink" => unlink::dispatch(sub_matches),
            _ => bail!("Unknown subcommand {}", name),
        }
    } else {
        bail!("No subcommand provided")
    }
}
