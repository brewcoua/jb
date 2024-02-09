use clap::{arg, Command, value_parser};
use jb::tool::{Tool, Kind, List};
use crate::emoji::*;

pub(crate) fn command() -> Command {
    Command::new("refresh")
        .about("Update a JetBrains tool to the latest version")
        .arg(
            arg!(tools: <TOOLS> "The tools to update")
                .required(true)
                .value_parser(value_parser!(Kind))
                .num_args(1..=10),
        )
        .arg(
            arg!(-f --force)
                .help("Force update, even if the tool is already up to date or not installed")
                .required(false),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> jb::Result<()> {
    let kinds = args.get_many::<Kind>("tools")
        .expect("Could not find argument tools")
        .map(Clone::clone);
    let force = args.get_flag("force");

    let mut error_batch = jb::Batch::new();
    let mut tools: Vec<Tool> = kinds.filter(|kind| {
            let tools = Tool::list_kind(*kind);
            if tools.is_err() || tools.unwrap().is_empty() {
                if force {
                    jb::warn!("No tools found for {kind}, but forcing update...");
                } else {
                    jb::warn!("No tools found for {kind}, skipping... {SKIP}");
                    return false;
                }
            }
            true
        })
        .map(Tool::from_kind)
        .collect();

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

    tools.sort(); tools.dedup();

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
            println!("{BIN} {tool}");
        }
    }

    jb::info!("{CHECK} Done!");
    for tool in tools {
        println!("{CIRCLE_ARROWS} {tool}");
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}