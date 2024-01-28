use anyhow::Result;
use clap::{arg, value_parser, Command};
use clap::parser::ValuesRef;
use colored::Colorize;
use tokio::task::JoinSet;
use jb_lib::tool::{Kind, Tool, Version};

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

pub(crate) async fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tool_kinds = args
        .get_many::<Kind>("tool")
        .expect("Could not find argument tools");
    let version: Option<&Version> = args.get_one::<Version>("build");
    let directory: Option<&std::path::PathBuf> = args.get_one::<std::path::PathBuf>("directory");

    install(
        tool_kinds,
        version.cloned(),
        directory.cloned(),
        args.get_flag("clean"),
    ).await?;

    Ok(())
}

async fn install(
    tool_kinds: ValuesRef<'_, Kind>,
    version: Option<Version>,
    directory: Option<std::path::PathBuf>,
    clean: bool,
) -> Result<()> {
    let mut set: JoinSet<Result<()>> = JoinSet::new();

    // Loop through all tools and concurrently install them
    for tool_kind in tool_kinds {
        let mut tool = Tool::new(*tool_kind);
        if version.is_some() {
            tool = tool.with_version(version.unwrap());
        }
        if directory.is_some() {
            tool = tool.with_directory(directory.clone().unwrap());
        }

        set.spawn(async move {
            tool.install().await?;

            log::info!(
                "Installed {} to {}",
                tool.kind.as_str().bright_green(),
                tool.as_path().display().to_string().bright_green()
            );

            if clean {
                // Clean up old versions
                log::info!(
                    "Cleaning up old versions of {}",
                    tool.kind.as_str().bright_green()
                );

                let installed_tools = Tool::list(tool.directory.clone())?
                    .into_iter()
                    .filter(|t| t.kind == tool.kind && t.version != tool.version)
                    .collect::<Vec<Tool>>();

                for tool in installed_tools {
                    tool.uninstall().await?;
                    log::info!(
                        "Uninstalled {}",
                        tool.as_path().display().to_string().bright_green()
                    );
                }

                log::info!(
                    "Cleaned up old versions of {}",
                    tool.kind.as_str().bright_green()
                );
            }

            Ok(())
        });
    }

    let mut error_count = 0;
    while let Some(result) = set.join_next().await {
        if let Err(e) = result {
            log::error!("{:?}", e);
            error_count += 1;
        }
    }

    if error_count > 0 {
        anyhow::bail!("{} errors occurred while installing", error_count);
    }
    Ok(())
}
