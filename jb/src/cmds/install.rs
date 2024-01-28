use std::sync::{Arc, Mutex};
use std::thread;

use clap::{arg, value_parser, Command};
use jb_lib::{tool::{Kind, Tool, Version}, error::{Result, Batch}, info_span, set_task, debug};

pub(crate) fn command() -> Command {
    Command::new("install")
        .about("Install a JetBrains tool")
        .arg(
            arg!(tool: <TOOL> "The tools to install")
                .required(true)
                .value_parser(value_parser!(Kind))
                .num_args(1..=10),
        )
        .arg(
            arg!(--build <VERSION>)
                .help("The release version to install (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(Version))
                .required(false),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to install the tool to")
                .value_parser(value_parser!(std::path::PathBuf))
                .required(false),
        )
        .arg(
            arg!(--clean)
                .help("Clean up old versions after installing")
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kinds = args
        .get_many::<Kind>("tool")
        .expect("Could not find argument tools");
    let version: Option<&Version> = args.get_one::<Version>("build");
    let directory: Option<&std::path::PathBuf> = args.get_one::<std::path::PathBuf>("directory");

    let clean: Arc<Mutex<bool>> = Arc::new(Mutex::new(args.get_flag("clean")));

    let handles: Vec<_> = tool_kinds
        .map(|tool_kind| {
            let mut tool = Tool::new(*tool_kind);
            if version.is_some() {
                tool = tool.with_version(version.unwrap().clone());
            }
            if directory.is_some() {
                tool = tool.with_directory(directory.unwrap().clone());
            }

            let clean = Arc::clone(&clean);

            thread::spawn(move || {
                set_task!(format!("install:{}", tool.kind.as_str()).as_str());

                let span = info_span!("Installing {}", tool.kind.as_str());

                tool.install()?;

                info_span!((span) "Installed {} to {}", tool.kind.as_str(), tool.as_path().display().to_string());

                if *clean.lock().unwrap() {
                    // Clean up old versions
                    let span = info_span!(
                        "Cleaning up old versions of {}",
                        tool.kind.as_str()
                    );

                    let installed_tools = Tool::list(tool.directory.clone())?
                        .into_iter()
                        .filter(|t| t.kind == tool.kind && t.version != tool.version)
                        .collect::<Vec<Tool>>();

                    for tool in installed_tools {
                        tool.uninstall()?;
                        debug!(
                            "Uninstalled {}",
                            tool.as_path().display().to_string()
                        );
                    }

                    info_span!((span) "Cleaned up old versions of {}", tool.kind.as_str());
                }

                Ok(())
            })
        }).collect();

    let mut error_batch = Batch::new();

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => error_batch.add(e),
            Err(e) => error_batch.add(
                e.downcast::<anyhow::Error>()
                    .expect("Could not cast thread error to anyhow::Error")
                    .context("Could not join thread"),
            ),
        }
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}
