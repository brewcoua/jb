use env_logger::{Builder, WriteStyle};

mod cmds;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

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
        .init();

    cmds::dispatch(matches.subcommand());
}
