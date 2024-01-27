use colored::Colorize;
use env_logger::Builder;
use std::io::Write;

mod cmds;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

    setup_logger(&matches);

    match cmds::dispatch(matches.subcommand()) {
        Ok(()) => {}
        Err(e) => {
            log::error!("{:?}", e);
            std::process::exit(1);
        }
    }
}

fn setup_logger(matches: &clap::ArgMatches) {
    let log_level = if matches.get_flag("verbose") {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };

    Builder::new()
        .filter(None, log_level)
        .format_timestamp(None)
        .format_module_path(false)
        .format(|buf, record| {
            let mut output = record.args().to_string();
            match record.level() {
                log::Level::Error => output = output.red().to_string(),
                log::Level::Warn => output = output.yellow().to_string(),
                log::Level::Info => output = output.green().to_string(),
                log::Level::Debug => output = output.blue().to_string(),
                log::Level::Trace => output = output.magenta().to_string(),
            }

            writeln!(
                buf,
                "{}{}{} {}",
                "[".dimmed().bold(),
                env!("CARGO_PKG_NAME").dimmed().bold(),
                "]".dimmed().bold(),
                output
            )
        })
        .init();
}
