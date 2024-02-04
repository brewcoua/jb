use tracing_subscriber::EnvFilter;
use tracing_subscriber::filter::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use jb::env::Variable;

mod cmds;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

    update_env(&matches);
    setup_logger();

    match cmds::dispatch(matches.subcommand()) {
        Ok(()) => {}
        Err(e) => {
            tracing::error!("{e}");
            std::process::exit(1);
        }
    }
}

fn update_env(matches: &clap::ArgMatches) {
    let notify = matches.get_flag("notify");
    if notify {
        Variable::Notify.set("true");
    }

    let tools_dir = matches.get_one::<std::path::PathBuf>("tools-dir");
    if let Some(tools_dir) = tools_dir {
        Variable::ToolsDirectory.set(tools_dir.to_str().unwrap().to_string());
    }

    let icons_dir = matches.get_one::<std::path::PathBuf>("icons-dir");
    if let Some(icons_dir) = icons_dir {
        Variable::IconsDirectory.set(icons_dir.to_str().unwrap().to_string());
    }

    let bin_dir = matches.get_one::<std::path::PathBuf>("bin-dir");
    if let Some(bin_dir) = bin_dir {
        Variable::BinariesDirectory.set(bin_dir.to_str().unwrap().to_string());
    }

    let verbose = matches.get_flag("verbose");
    if verbose {
        Variable::Verbose.set("true");
    }
}

fn setup_logger() {
    let verbose = Variable::Verbose.get::<String>().parse::<bool>().unwrap_or(false);

    let log_level = if verbose {
        LevelFilter::DEBUG
    } else {
        LevelFilter::INFO
    };

    let filter = EnvFilter::from_env("JB_LOG") // Ignore all tls and reqwest logs
        .add_directive("hyper=off".parse().unwrap())
        .add_directive("rustls=off".parse().unwrap())
        .add_directive("h2=off".parse().unwrap())
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
