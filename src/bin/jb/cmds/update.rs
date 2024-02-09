use anyhow::Context;
use clap::{Command,arg};
use crate::update::Release;
use crate::emoji::CHECK;
use termimad::crossterm::style::Color;

pub(crate) fn command() -> Command {
    Command::new("update")
        .about("Update the CLI to the latest version")
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
                jb::info!("{CHECK} Updated to the latest version");
                if !changelog.is_empty() {
                    let mut skin = termimad::MadSkin::default();
                    skin.bold.set_fg(Color::AnsiValue(208));
                    skin.italic.set_fg(Color::AnsiValue(208));
                    skin.set_headers_fg(Color::Cyan);

                    skin.print_text(&changelog);
                }
            }

            Ok(())
        }
        Err(err) => jb::bail!(err),
    }
}