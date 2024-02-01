use std::sync::Arc;
use std::thread;
use anyhow::anyhow;

use clap::{arg, value_parser, Command};
use jb_lib::{tool::{Kind, Tool, Version}, error::{Result, Batch}};

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

    let clean = Arc::new(args.get_flag("clean"));

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
                let span = tracing::info_span!("task", tool = tool.name());
                let _guard = span.enter();

                tool.install()?;

                tracing::info!("Installed {}", tool.as_path().display().to_string());

                if *clean {
                    // Clean up old versions
                    let span = tracing::info_span!("cleanup", tool = tool.name());
                    let _guard = span.enter();

                    tracing::info!("Cleaning up old versions of {}", tool.kind.as_str());

                    let installed_tools = Tool::list(tool.directory.clone())?
                        .into_iter()
                        .filter(|t| t.kind == tool.kind && t.version != tool.version)
                        .collect::<Vec<Tool>>();

                    for tool in installed_tools {
                        tool.uninstall()?;
                        tracing::debug!(
                            "Uninstalled {}",
                            tool.as_path().display().to_string()
                        );
                    }

                    tracing::info!("Cleaned up old versions of {}", tool.kind.as_str());
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
            Err(e) => error_batch.add(anyhow!("Thread panicked: {:?}", e)),
        }
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}
