use std::backtrace::BacktraceStatus;
use std::io::Write;
use env_logger::{Builder, WriteStyle};
use colored::Colorize;

mod cmds;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

    setup_logger(&matches);

    match cmds::dispatch(matches.subcommand()) {
        Ok(_) => {}
        Err(e) => {
            log::error!("{}", e);

            let backtrace = e.backtrace();
            if let BacktraceStatus::Captured = backtrace.status() {
                log::error!("{}", backtrace);
            }

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
    let write_style = match matches.get_one::<String>("color").map(|s| s.as_str()) {
        Some("always") => WriteStyle::Always,
        Some("never") => WriteStyle::Never,
        Some("auto") => WriteStyle::Auto,
        _ => WriteStyle::Auto,
    };

    Builder::new()
        .filter(None, log_level)
        .format_timestamp(None)
        .format_module_path(false)
        .write_style(write_style)
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
                "jb-tool".green().bold(),
                "]".dimmed().bold(),
                output
            )
        })
        .init();
}