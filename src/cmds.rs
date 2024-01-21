mod install;
mod refresh;
mod list;
mod uninstall;

use clap::{arg, Command};

pub fn cli() -> Command {
    Command::new("jb-cli")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A simple CLI for JetBrains' IDEs and tools")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            arg!(-v --verbose)
                .help("Enable verbose logging")
                .required(false)
                .global(true)
        )
        .arg(
            arg!(--color <WHEN>)
                .help("When to use color in output")
                .value_parser(["always", "never", "auto"])
                .num_args(0..=1)
                .require_equals(true)
                .default_value("auto")
                .default_missing_value("always")
                .global(true)
        )
        .subcommand(install::command())
        .subcommand(list::command())
}

pub fn dispatch(args: Option<(&str, &clap::ArgMatches)>) {
    if let Some((name, sub_matches)) = args {
        match name {
            "install" => install::dispatch(sub_matches),
            "list" => list::dispatch(sub_matches),
            _ => {
                log::error!("Unknown subcommand {}", name);
                std::process::exit(1);
            }
        }
    }

    std::process::exit(0);
}
