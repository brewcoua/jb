use std::thread;
use clap::{arg, value_parser, Command};
use jb::{Tool, Result, Batch};
use jb::tool::{Install, List};

pub(crate) fn command() -> Command {
    Command::new("uninstall")
        .about("Uninstall JetBrains tools")
        .arg(
            arg!(tools: <TOOLS> "The tools to uninstall")
                .required(true)
                .value_parser(value_parser!(Tool))
                .num_args(1..=10),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to uninstall the tool from")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let args_tools = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tool");

    let mut tools: Vec<Tool> = Vec::new();
    let installed_tools = match Tool::list() {
        Ok(tools) => tools,
        Err(e) => jb::bail_with!(e, "Could not list installed tools"),
    };

    let mut error_batch = Batch::new();

    for tool in args_tools {
        if tool.version.is_none() {
            let installed_tools = installed_tools.clone()
                .into_iter()
                .filter(|t| t.kind == tool.kind)
                .collect::<Vec<Tool>>();

            if installed_tools.is_empty() {
                jb::batch_with!(error_batch, "Could not find any installed versions of {tool}");
            } else {
                tools.extend(installed_tools);
            }
        } else {
            let installed_tools = installed_tools.clone()
                .into_iter()
                .filter(|t| tool.matched(t))
                .collect::<Vec<Tool>>();

            if installed_tools.is_empty() {
                jb::batch_with!(error_batch, "Could not find any installed versions of {tool}");
            } else {
                tools.extend(installed_tools);
            }
        }
    }

    if !error_batch.is_empty() {
        return Err(error_batch);
    }

    tools.sort(); tools.dedup();

    let handles: Vec<_> = tools
        .iter()
        .map(|tool| {
            let tool = tool.clone();

            thread::spawn(move || {
                tool.uninstall()?;

                Ok(())
            })
        }).collect();

    let mut error_batch = Batch::new();

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => error_batch.add(e),
            Err(e) => jb::batch_with!(error_batch, "Thread panicked: {e:?}"),
        }
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}
