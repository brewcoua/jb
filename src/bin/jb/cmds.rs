mod install;
mod link;
mod list;
mod uninstall;
mod unlink;

use anyhow::anyhow;
use clap::{arg, Arg, Command, value_parser};
use jb::error::{Batch, Result};

pub fn cli() -> Command {
    Command::new("jb")
        .version(env!("CARGO_PKG_VERSION"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            arg!(-v --verbose)
                .help("Enable verbose logging")
                .required(false)
                .global(true),
        )
        .arg(
            arg!(-n --notify)
                .help("Enable desktop notifications (enabled by default in non-tty)")
                .required(false)
                .global(true),
        )
        .arg(
            Arg::new("tools-dir")
                .long("tools-dir")
                .help("The directory to install tools to")
                .required(false)
                .global(true)
                .value_parser(value_parser!(std::path::PathBuf))
                .num_args(1),
        )
        .arg(
            Arg::new("icons-dir")
                .long("icons-dir")
                .help("The directory to link icons to")
                .required(false)
                .global(true)
                .value_parser(value_parser!(std::path::PathBuf))
                .num_args(1),
        )
        .arg(
            Arg::new("bin-dir")
                .long("bin-dir")
                .help("The directory to link binaries to")
                .required(false)
                .global(true)
                .value_parser(value_parser!(std::path::PathBuf))
                .num_args(1),
        )
        .subcommand(install::command())
        .subcommand(uninstall::command())
        .subcommand(list::command())
        .subcommand(link::command())
        .subcommand(unlink::command())
}

pub(crate) fn dispatch(args: Option<(&str, &clap::ArgMatches)>) -> Result<()> {
    if let Some((name, sub_matches)) = args {
        match name {
            "install" => install::dispatch(sub_matches),
            "uninstall" => uninstall::dispatch(sub_matches),
            "list" => list::dispatch(),
            "link" => link::dispatch(sub_matches),
            "unlink" => unlink::dispatch(sub_matches),
            _ => Err(Batch::from(anyhow!("Unknown subcommand {} provided", name))),
        }
    } else {
        Err(Batch::from(anyhow!("No subcommand provided")))
    }
}
