//! Module for the `jb` command line interface

/// Commands
mod install;
mod uninstall;
mod refresh;
mod link;
mod unlink;
mod list;
mod update;
mod cd;
mod info;
mod meta;


use clap::{arg, Arg, Command, value_parser};
use jb::error::Result;

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
        .subcommand(refresh::command())
        .subcommand(list::command())
        .subcommand(link::command())
        .subcommand(unlink::command())
        .subcommand(update::command())
        .subcommand(cd::command())
        .subcommand(info::command())
        .subcommand(meta::command())
}

pub(crate) fn dispatch(args: Option<(&str, &clap::ArgMatches)>) -> Result<()> {
    if let Some((name, sub_matches)) = args {
        match name {
            "install" => install::dispatch(sub_matches),
            "uninstall" => uninstall::dispatch(sub_matches),
            "refresh" => refresh::dispatch(sub_matches),
            "list" => list::dispatch(),
            "link" => link::dispatch(sub_matches),
            "unlink" => unlink::dispatch(sub_matches),
            "update" => update::dispatch(sub_matches),
            "cd" => cd::dispatch(),
            "info" => {
                info::dispatch();
                Ok(())
            },
            "meta" => meta::dispatch(),
            _ => jb::bail!("Unknown subcommand {} provided", name),
        }
    } else {
        jb::bail!("No subcommand provided")
    }
}
