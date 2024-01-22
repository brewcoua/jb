use clap::{arg, value_parser, Command};
use jb_lib::tool::{Tool, Kind, ReleaseVersion};

pub(crate) fn command() -> Command {
    Command::new("uninstall")
        .about("Uninstall a tool, removing the linked version or a specific version")
        .arg(
            arg!(tool: <TOOL> "The tool to uninstall")
                .required(true)
                .value_parser(value_parser!(Kind))
        )
        .arg(
            arg!(--build <VERSION>)
                .help("The release version to uninstall (e.g. '2023.2.1-eap' or 'preview')")
                .value_parser(value_parser!(ReleaseVersion))
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to uninstall the tool from")
                .value_parser(value_parser!(std::path::PathBuf))
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) {
    let tool_kind: Kind = args.get_one::<Kind>("tool")
        .expect("Could not find argument tool")
        .clone();
    let directory = args.get_one::<std::path::PathBuf>("directory");
    let version = args.get_one::<ReleaseVersion>("build");

    let mut tool = Tool::new(tool_kind);
    if !directory.is_none() {
        tool = tool.with_directory(directory.unwrap().clone());
    }

    tool = match version {
        Some(v) => tool.with_version(v.clone()),
        None => {
            let installed_tools = Tool::list(tool.directory().clone());
            if let Err(e) = installed_tools {
                log::error!("Failed to list installed tools:\n{}", e);
                std::process::exit(1);
            }

            let installed_tools = installed_tools.unwrap()
                .into_iter()
                .filter(|t| t.kind() == tool.kind())
                .collect::<Vec<Tool>>();

            if installed_tools.len() == 0 {
                log::error!("No installed versions of {} found", tool.kind().as_str());
                std::process::exit(1);
            } else if installed_tools.len() == 1 {
                // No need to search for linked version
                installed_tools[0].clone()
            } else {
                // Find the one that is linked
                let linked_tool = installed_tools.iter()
                    .find(|t| t.is_linked());

                if linked_tool.is_none() {
                    log::error!("Could not find linked version of {}", tool.kind().as_str());
                    std::process::exit(1);
                } else {
                    linked_tool.unwrap().clone()
                }
            }
        }
    };

    match tool.uninstall() {
        Ok(_) => log::info!("Uninstalled {} from {}", tool.kind().as_str(), tool.as_path().display()),
        Err(e) => {
            log::error!("Failed to uninstall tool:\n{}", e);
            std::process::exit(1);
        }
    }
}