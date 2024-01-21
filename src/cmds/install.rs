use clap::{arg, value_parser, Command};
use jb_cli::tools::{Tool, release::ReleaseType};

pub(crate) fn command() -> Command {
    Command::new("install")
        .about("Install any of JetBrains' IDEs and tools")
        .arg(
            arg!(tool: <TOOL> "The tool to install")
                .required(true)
                .value_parser(value_parser!(Tool))
        )
        .arg(
            arg!(--type <TYPE>)
                .help("The release type to install (e.g. release, eap, preview)")
                .value_parser(value_parser!(ReleaseType))
        )
        .arg(
            arg!(-d --directory <PATH>)
                .help("The directory to install the tool to")
                .value_parser(value_parser!(std::path::PathBuf))
        )
        .arg(
            arg!(--noclean)
                .help("Do not clean up old versions of the tool")
                .required(false)
        )
}

pub(crate) fn dispatch(args: &clap::ArgMatches) {
    let tool: &Tool = args.get_one::<Tool>("tool").expect("Could not find argument tool");
    let release_type = args.get_one::<ReleaseType>("type");

    let directory = args.get_one::<std::path::PathBuf>("directory");
    let directory = directory.unwrap_or(
        &std::path::PathBuf::from(
            std::env::var("HOME")
                .expect("Failed to get home directory, please use -d/--directory to specify a directory")
        ).join(".local/share/JetBrains")
    ).clone();

    // Get all folders matching the tool name in the given directory (directory/apps/tool-*)
    let leftover_versions = std::fs::read_dir(directory.join("apps"))
        .expect("Failed to read apps directory")
        .filter_map(|entry| {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();

            if path.is_dir() {
                let name = path.file_name().expect("Failed to get file name").to_str().expect("Failed to convert file name to string");
                if name.starts_with(tool.as_str()) {
                    Some(path)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect::<Vec<_>>();


    let result = jb_cli::tools::install::ToolInstaller::new(tool.clone(), release_type, directory)
        .install()
        .expect("Failed to install tool");

    if result && !args.get_flag("noclean") {
        for path in leftover_versions {
            log::info!("Cleaning up old version {}", path.display());
            std::fs::remove_dir_all(path).expect("Failed to remove old version");
        }
    }
}
