use anyhow::{anyhow};
use clap::{arg, value_parser, Command};
use colored::Colorize;
use jb_lib::{tool::{Kind, Tool, Version}, error::{Result, Batch}};

pub(crate) fn command() -> Command {
    Command::new("uninstall")
        .about("Uninstall a JetBrains tool")
        .arg(
            arg!(tool: <TOOL> "The tool to uninstall")
                .required(true)
                .value_parser(value_parser!(Kind)),
        )
        .arg(
            arg!(--build <VERSION>)
                .help("The release version to uninstall (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(Version)),
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to uninstall the tool from")
                .value_parser(value_parser!(std::path::PathBuf)),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kind = args
        .get_one::<Kind>("tool")
        .expect("Could not find argument tool");
    let directory = args.get_one::<std::path::PathBuf>("directory");
    let version = args.get_one::<Version>("build");

    let mut tool = Tool::new(*tool_kind);
    if directory.is_some() {
        tool = tool.with_directory(directory.unwrap().clone());
    }

    tool = if let Some(v) = version {
        tool.with_version(*v)
    } else {
        let installed_tools = match Tool::list(tool.directory.clone()) {
            Ok(tools) => tools,
            Err(e) => {
                return Err(
                    Batch::from(
                        e.context(format!("Could not list installed versions of {}", tool.kind.as_str()))
                    )
                );
            }
        };


        let installed_tools = installed_tools
            .into_iter()
            .filter(|t| t.kind == tool.kind)
            .collect::<Vec<Tool>>();

        if installed_tools.is_empty() {
            return Err(Batch::from(anyhow!(
                "Could not find any installed versions of {}",
                tool.kind.as_str()
            )));
        } else if installed_tools.len() == 1 {
            // No need to search for linked version
            installed_tools[0].clone()
        } else {
            // Find the one that is linked
            let linked_tool = installed_tools.iter().find(|t| t.is_linked());

            match linked_tool {
                Some(t) => t.clone(),
                None => Err(Batch::from(anyhow!(
                    "Could not find a linked version of {}",
                    tool.kind.as_str()
                )))?,
            }
        }
    };

    match tool.uninstall() {
        Ok(()) => {}
        Err(e) => {
            return Err(
                Batch::from(
                    e.context(format!("Could not uninstall {}", tool.as_path().display()))
                )
            );
        }
    }

    log::info!(
        "Uninstalled {} from {}",
        tool.kind.as_str().bright_green(),
        tool.as_path().display().to_string().bright_green()
    );

    Ok(())
}
