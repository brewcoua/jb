use clap::{arg, value_parser, Command};
use jb::{Tool, Result, Batch};
use jb::tool::{Link, List};
use crate::emoji::*;

pub(crate) fn command() -> Command {
    Command::new("uninstall")
        .about("Uninstall JetBrains tools")
        .arg(
            arg!(tools: <TOOLS> "The tools to uninstall")
                .required(true)
                .value_parser(value_parser!(Tool))
                .num_args(1..=10),
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let args_tools: Vec<_> = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tool")
        .map(Clone::clone)
        .collect();

    let mut error_batch = Batch::new();

    // First step, list all tools that match the given tools
    jb::info!("{LOOKING_GLASS} Searching for matching tools...");
    let mut tools: Vec<_> = crate::concurrent_step!(error_batch, args_tools, |tool: Tool| {
        let matched_tools = tool.list_matching()?;
        Ok(matched_tools)
    }).into_iter().flatten().collect();

    tools.sort(); tools.dedup();

    if tools.is_empty() {
        jb::bail!("No tools found, nothing to uninstall");
    }

    // Second step, unlink all tools
    jb::info!("{LINK} Unlinking tools...");
    let tools = crate::concurrent_step!(error_batch, tools, |tool: Tool| {
        if tool.is_linked() {
            tool.unlink()?;
        }
        Ok(tool)
    });

    if tools.is_empty() {
        jb::warn!("No tools left to uninstall, skipping... {SKIP}");
        return if error_batch.is_empty() {
            Ok(())
        } else {
            Err(error_batch)
        }
    }

    // Third step, uninstall all tools
    jb::info!("{BIN} Uninstalling tools...");
    let tools = crate::concurrent_step!(error_batch, tools, |tool: Tool| {
        std::fs::remove_dir_all(tool.as_path())?;
        Ok(tool)
    });

    if tools.is_empty() {
        jb::warn!("No tools left to uninstall, skipping... {SKIP}");
        return if error_batch.is_empty() {
            Ok(())
        } else {
            Err(error_batch)
        }
    }

    jb::info!("{CHECK} Uninstalled all tools:");
    for tool in tools {
        println!("- {tool}");
    }

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}
