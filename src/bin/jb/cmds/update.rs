use anyhow::Context;
use clap::{Command,arg};
use crate::update::Release;

pub(crate) fn command() -> Command {
    Command::new("update")
        .about("Update the CLI")
        .long_about("Update the CLI to the latest version")
        .after_help(
            "This command will update the CLI to the latest version. \
            It will download the latest version and replace the current version with it.",
        )
        .arg(
            arg!(-f --force)
                .help("Force the update, even if the latest version is already installed")
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> jb::error::Result<()> {
    let force = args.get_flag("force");

    let location = match std::env::current_exe() {
        Ok(location) => location,
        Err(err) => jb::bail!(err),
    };

    match Release::try_update(&location, force)
            .with_context(|| "Failed to update the CLI") {
        Ok((done, changelog)) => {
            if done {
                tracing::info!("Updated to the latest version");
                if !changelog.is_empty() {
                    println!("\nChangelog:\n{changelog}");
                }
            }

            Ok(())
        }
        Err(err) => jb::bail!(err),
    }
}