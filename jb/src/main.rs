use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use jb_lib::env::Variable;

mod cmds;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

    setup_logger(&matches);

    match cmds::dispatch(matches.subcommand()) {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("{e}");
            std::process::exit(1);
        }
    }
}

fn setup_logger(matches: &clap::ArgMatches) {
    let verbose = matches.get_flag("verbose");
    let env_verbose = Variable::get::<String>(Variable::Verbose).parse::<bool>().unwrap_or(false);

    let log_level = if verbose || env_verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let filter = EnvFilter::from_env("JB_LOG") // Ignore hyper and reqwest logs
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("reqwest=off".parse().unwrap())
        .add_directive(log_level.into());

    let fmt = tracing_subscriber::fmt::layer()
        .with_ansi(std::env::var("TERM").ok() == Some("xterm-256color".to_string()))
        // Show spans but not targets
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
        .with_target(false)
        .with_level(true);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt)
        .init();
}
