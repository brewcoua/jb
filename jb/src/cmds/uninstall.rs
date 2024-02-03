use std::thread;
use clap::{arg, value_parser, Command};
use jb_lib::{tool::{Tool, release}, error::{Result, Batch}};

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
    let directory = args.get_one::<std::path::PathBuf>("directory");

    let mut tools: Vec<Tool> = Vec::new();
    let installed_tools = match Tool::list(directory.cloned()) {
        Ok(tools) => tools,
        Err(e) => return Err(Batch::from(e)),
    };

    let mut error_batch = Batch::new();

    for tool in args_tools {
        if tool.version.is_none() {
            let installed_tools = installed_tools.clone()
                .into_iter()
                .filter(|t| t.kind == tool.kind)
                .collect::<Vec<Tool>>();

            if installed_tools.is_empty() {
                error_batch.add(anyhow::anyhow!("Could not find any installed versions of {}", tool.kind.as_str()));
            } else {
                tools.extend(installed_tools);
            }
        } else {
            let version = tool.version.as_ref().unwrap();

            if version.is_complete() {
                tools.push(tool.clone());
                continue;
            }

            // Match against the version
            let installed_tools = installed_tools.clone()
                .into_iter()
                .filter(|t| {
                    if t.kind != tool.kind || t.version.is_none() {
                        return false;
                    }

                    let t = t.version.unwrap();

                    if version.major.is_some() && (t.major.is_none() || t.major.unwrap() != version.major.unwrap()) {
                        return false;
                    }
                    if version.minor.is_some() && (t.minor.is_none() || t.minor.unwrap() != version.minor.unwrap()) {
                        return false;
                    }
                    if version.patch.is_some() && (t.patch.is_none() || t.patch.unwrap() != version.patch.unwrap()) {
                        return false;
                    }
                    if version.release != release::Type::Release && (t.release != version.release) {
                        return false;
                    }

                    true
                })
                .collect::<Vec<Tool>>();

            if installed_tools.is_empty() {
                error_batch.add(anyhow::anyhow!("Could not find any installed versions of {tool}"));
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
            let mut tool = tool.clone();
            if directory.is_some() {
                tool = tool.with_directory(directory.cloned().unwrap());
            }

            thread::spawn(move || {
                let span = tracing::info_span!("task", tool = tool.to_string());
                let _guard = span.enter();

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
        Err(error_batch)
    }
}
