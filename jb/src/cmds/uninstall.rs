use std::thread;
use clap::{arg, value_parser, Command};
use jb_lib::{tool::Tool, error::{Result, Batch}};

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
    let tools = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tool");
    let directory = args.get_one::<std::path::PathBuf>("directory");

    let handles: Vec<_> = tools
        .map(|tool| {
            let mut tool = tool.clone();
            if directory.is_some() {
                tool = tool.with_directory(directory.unwrap().clone());
            }

            thread::spawn(move || {
                let span = tracing::info_span!("task", tool = tool.to_string());
                let _guard = span.enter();

                if tool.version.is_none() {
                    // Find the linked version
                    let installed_tools = Tool::list(tool.directory.clone())?
                        .into_iter()
                        .filter(|t| t.kind == tool.kind)
                        .collect::<Vec<Tool>>();

                    if installed_tools.is_empty() {
                        anyhow::bail!("Could not find any installed versions of {}", tool.kind.as_str());
                    } else if installed_tools.len() == 1 {
                        // No need to search for linked version
                        tool = installed_tools[0].clone();
                    } else {
                        // Find the one that is linked
                        let linked_tool = installed_tools.iter().find(|t| t.is_linked());

                        match linked_tool {
                            Some(t) => tool = t.clone(),
                            None => anyhow::bail!("Could not find a linked version of {}", tool.kind.as_str()),
                        }
                    }
                }

                tool.uninstall()?;

                tracing::info!(
                    "Uninstalled {tool} from {}",
                    tool.as_path().display(),
                );

                Ok(())
            })
        }).collect();

    let mut error_batch = Batch::new();

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => error_batch.add(e),
            Err(e) => error_batch.add(anyhow::anyhow!("Thread panicked: {e:?}")),
        }
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch.into())
    }
}
