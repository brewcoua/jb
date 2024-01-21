use clap::Parser;
use env_logger::{Builder, WriteStyle};
use jb_cli::{JetBrainsCLI, dispatch };

fn main() {
    Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format_timestamp(None)
        .format_module_path(false)
        .write_style(WriteStyle::Always)
        .init();

    let args = JetBrainsCLI::parse();


    dispatch(args);
}
