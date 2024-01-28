mod install;
mod link;
mod list;
mod uninstall;
mod unlink;

use anyhow::anyhow;
use clap::{arg, Command};
use jb_lib::error::{Batch, Result};

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

pub(super) async fn dispatch(args: Option<(&str, &clap::ArgMatches)>) -> Result<()> {
    if let Some((name, sub_matches)) = args {
        match name {
            "install" => install::dispatch(sub_matches),
            "uninstall" => uninstall::dispatch(sub_matches),
            "list" => list::dispatch(sub_matches),
            "link" => link::dispatch(sub_matches),
            "unlink" => unlink::dispatch(sub_matches),
            _ => Err(Batch::from(anyhow!("Unknown subcommand {} provided", name))),
        }
    } else {
        Err(Batch::from(anyhow!("No subcommand provided")))
    }
}
