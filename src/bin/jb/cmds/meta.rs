use clap::Command;
use serde::Serialize;
use jb::Tool;
use jb::tool::{Kind, List};

pub(crate) fn command() -> Command {
    Command::new("meta")
        .about("Print metadata about the current environment and available tools in JSON format")
}

pub(crate) fn dispatch() -> jb::Result<()> {
    let run = || {
        let meta = Meta {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            env: MetaEnv {
                verbose: jb::env::Variable::Verbose.get_bool(),
                notify: jb::env::Variable::Notify.get_bool(),
                tools_dir: jb::env::Variable::ToolsDirectory.get::<std::path::PathBuf>(),
                icons_dir: jb::env::Variable::IconsDirectory.get::<std::path::PathBuf>(),
                bin_dir: jb::env::Variable::BinariesDirectory.get::<std::path::PathBuf>(),
            },
            kinds: Kind::list().to_vec(),
            tools: Tool::list()?,
        };

        println!("{}", serde_json::to_string_pretty(&meta)?);

        Ok::<(), anyhow::Error>(())
    };

    match run() {
        Ok(()) => (),
        Err(err) => jb::bail!(err),
    }

    Ok(())
}

#[derive(Serialize, Debug)]
struct Meta {
    name: &'static str,
    version: &'static str,
    env: MetaEnv,
    kinds: Vec<Kind>,
    tools: Vec<Tool>,
}

#[derive(Serialize, Debug)]
struct MetaEnv {
    verbose: bool,
    notify: bool,
    tools_dir: std::path::PathBuf,
    icons_dir: std::path::PathBuf,
    bin_dir: std::path::PathBuf,
}