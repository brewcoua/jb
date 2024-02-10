mod set;
mod status;
mod enable;
mod disable;
mod start;

use clap::Command;

pub(crate) fn command() -> Command {
    Command::new("auto")
        .about("Manage automatic updates for tools")
        .subcommand(set::command())
        .subcommand(status::command())
        .subcommand(enable::command())
        .subcommand(disable::command())
        .subcommand(start::command())
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> jb::Result<()> {
    if let Some(subcommand) = args.subcommand() {
        match subcommand.0 {
            "set" => set::dispatch(subcommand.1),
            "status" => status::dispatch(),
            "enable" => enable::dispatch(),
            "disable" => disable::dispatch(),
            "start" => start::dispatch(),
            _ => unreachable!(),
        }
    } else {
        jb::bail!("No subcommand provided")
    }
}
