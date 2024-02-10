use clap::{arg, Command, value_parser};
use jb::tool::{Tool, Kind, List};
use crate::emoji::*;

pub(crate) fn command() -> Command {
    Command::new("refresh")
        .about("Update a JetBrains tool to the latest version")
        .arg(
            arg!(tools: <TOOLS> "The tools to update")
                .required(false)
                .value_parser(value_parser!(Kind))
                .num_args(1..=10),
        )
        .arg(
            arg!(-f --force)
                .help("Force update, even if the tool is already up to date")
                .required(false),
        )
        .arg(
            arg!(-i --install)
                .help("Install the tool if it is not already installed")
                .required(false),
        )
        .arg(
            arg!(--all)
                .help("Apply the command to all installed tools (overrides the tools argument)")
                .long_help("Apply the command to all installed tools (overrides the tools argument)\nIt will not install any tools that are not already installed, regardless of the --install flag")
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> jb::Result<()> {
    let kinds = args.get_many::<Kind>("tools");
    let all = args.get_flag("all");
    let force = args.get_flag("force");
    let install = args.get_flag("install");

    if kinds.is_none() && !all {
        jb::bail!("No tools specified, nothing to update");
    } else if all && kinds.is_some() {
        jb::warn!("Ignoring tools argument, --all flag is set... {SKIP}");
    }

    let mut error_batch = jb::Batch::new();
    let mut tools: Vec<Tool>;

    if all { // If the --all flag is set, get all installed tools
        let mut installed_tools = jb::catch!(Tool::list());
        installed_tools.sort_by(|a, b| a.kind.cmp(&b.kind));
        installed_tools.dedup_by(|a, b| a.kind == b.kind);

        tools = installed_tools
            .iter()
            .map(|tool| Tool::from_kind(tool.kind))
            .collect();
    } else { // Otherwise, get the tools from the arguments
        tools = kinds.unwrap().map(Clone::clone)
            .filter(|kind| {
                let tools = Tool::list_kind(*kind);
                if tools.is_err() || tools.unwrap().is_empty() {
                    if install {
                        jb::warn!("No tools found for {kind}, but installing anyway...");
                    } else {
                        jb::warn!("No tools found for {kind}, skipping... {SKIP}");
                        return false;
                    }
                }
                true
            })
            .map(Tool::from_kind)
            .collect();

        tools.sort(); tools.dedup();
    }

    let mut old_tools: Vec<Tool> = tools
        .iter()
        .filter_map(|tool| {
            // Either find the linked tool for this kind, or the latest version
            let linked = tool.kind.linked().unwrap();
            if let Some(linked) = linked {
                Some(linked)
            } else {
                tool.kind.latest().unwrap()
            }
        })
        .collect();

    if tools.is_empty() {
        jb::bail!("No tools found, nothing to update");
    }

    tools = crate::util::install_tools(&mut error_batch, tools, force);

    if tools.is_empty() {
        jb::warn!("No tools left to update, exiting... {SKIP}");
        return if error_batch.is_empty() {
            Ok(())
        } else {
            Err(error_batch)
        };
    }

    // Filter out the old tools that failed to update
    old_tools.retain(|tool| {
        tools.iter().any(|new_tool| new_tool.kind == tool.kind)
    });

    if old_tools.is_empty() {
        jb::info!("{CLEAN} No old versions to clean up, skipping... {SKIP}");
    } else {
        jb::info!("{CLEAN} Cleaning up old versions...");
        for tool in old_tools {
            if let Err(err) = std::fs::remove_dir_all(tool.as_path()) {
                jb::warn!("Failed to clean up {tool}, skipping... {SKIP}");
                error_batch.add(err.into());
            }
            println!("{FIRECRACKER} {tool}");
        }
    }

    let notify = jb::env::Variable::Notify.get_bool();

    jb::info!("{CHECK} Done!");
    for tool in tools {
        println!("{CIRCLE_ARROWS} {tool}");
        if notify {
            jb::catch!(
                jb::notify(
                    &format!("Updated {tool} to the latest version"),
                    tool.as_icon().to_str().unwrap(),
                )
            );
        }
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}