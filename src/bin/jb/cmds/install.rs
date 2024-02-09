use anyhow::Context;
use clap::{arg, value_parser, Command};
use jb::{Tool, Result, Batch};
use jb::env::Variable;
use jb::tool::{List};
use crate::emoji::*;

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
            arg!(-c --clean)
                .help("Clean up old versions after installing")
                .required(false),
        )
        .arg(
            arg!(-f --force)
                .help("Force installation, even if the tool is already installed")
                .required(false),
        )
}

#[allow(clippy::too_many_lines)]
pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tools: Vec<Tool> = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tools")
        .map(Clone::clone)
        .collect();

    let clean = args.get_flag("clean");
    let force = args.get_flag("force");
    let mut error_batch = Batch::new();

    let tools = crate::util::install_tools(&mut error_batch, tools, force);

    if tools.is_empty() {
        jb::warn!("No tools left to install, exiting... {SKIP}");
        return if error_batch.is_empty() {
            Ok(())
        } else {
            Err(error_batch)
        };
    }

    if clean {
        let old_tools = Tool::list();
        if let Err(e) = old_tools {
            jb::warn!("Failed to list installed tools, skipping cleanup... {SKIP}");
            jb::batch_with!(error_batch, e);
        } else {
            let kinds = tools.iter().map(|tool| tool.kind).collect::<Vec<_>>();
            let old_tools = old_tools
                .unwrap()
                .into_iter()
                .filter(|tool| kinds.contains(&tool.kind) && !tools.contains(tool))
                .collect::<Vec<_>>();

            if old_tools.is_empty() {
                jb::info!("{CLEAN} No old versions to clean up, skipping... {SKIP}");
            } else {
                jb::info!("{CLEAN} Cleaning up old versions...");

                let cleaned_tools = crate::concurrent_step!(error_batch, old_tools, |tool: Tool| {
                    jb::make!("{}", tool.as_str());

                    let path = tool.as_path();
                    let result = std::fs::remove_dir_all(path)
                        .with_context(|| format!("Failed to clean {}", tool.as_str()));

                    if let Err(e) = result {
                        jb::warn!("Failed to clean {}, skipping... {SKIP}", tool.as_str());
                        return Err(e);
                    }

                    Ok(tool)
                });

                for cleaned_tool in cleaned_tools {
                    println!("{WASTEBASKET} {cleaned_tool}");
                }
            }
        }
    }

    let notify = Variable::Notify.get_bool();

    jb::info!("{CHECK} Done!");
    for tool in tools {
        println!("{PACKAGE} {tool}");
        if notify {
            jb::catch_with!(
                error_batch,
                jb::notify(
                    &format!("Installed {tool}"),
                    tool.as_icon().to_str().unwrap()
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

