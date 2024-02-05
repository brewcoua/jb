use std::sync::Arc;
use std::thread;
use anyhow::anyhow;

use clap::{arg, value_parser, Command};
use jb::{Tool, Result, Batch};
use jb::tool::{Install,List};

pub(crate) fn command() -> Command {
    Command::new("install")
        .about("Install JetBrains tools")
        .arg(
            arg!(tools: <TOOLS> "The tools to install")
                .required(true)
                .value_parser(value_parser!(Tool))
                .num_args(1..=10),
        )
        .arg(
            arg!(--clean)
                .help("Clean up old versions after installing")
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tools = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tools");

    let clean = Arc::new(args.get_flag("clean"));

    let handles: Vec<_> = tools
        .map(|tool| {
            let mut tool = tool.clone();

            let clean = Arc::clone(&clean);

            thread::spawn(move || {
                let span = tracing::info_span!("task", tool = tool.as_str());
                let _guard = span.enter();

                tool.install()?;

                tracing::info!("Installed {}", tool.as_str());

                if *clean {
                    // Clean up old versions
                    let span = tracing::info_span!("cleanup");
                    let _guard = span.enter();

                    tracing::info!("Cleaning up older versions of {}", tool.kind);

                    let mut installed_tools = Tool::list_kind(tool.kind)?;
                    installed_tools.sort_by(|a, b| b.version.cmp(&a.version));
                    installed_tools.retain(|t| t.version < tool.version);

                    for tool in installed_tools {
                        tool.uninstall()?;
                    }

                    tracing::info!("Cleaned up older versions of {}", tool.kind);
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
