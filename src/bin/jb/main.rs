use jb::env::Variable;

mod cmds;
mod update;

fn main() {
    let cli = cmds::cli();
    let matches = cli.get_matches();

    update_env(&matches);
    match cmds::dispatch(matches.subcommand()) {
        Ok(()) => {}
        Err(e) => {
            jb::error!("{e}");
            std::process::exit(1);
        }
    }
}

fn update_env(matches: &clap::ArgMatches) {
    let notify = matches.get_flag("notify");
    if notify {
        Variable::Notify.set("true");
    }

    let tools_dir = matches.get_one::<std::path::PathBuf>("tools-dir");
    if let Some(tools_dir) = tools_dir {
        Variable::ToolsDirectory.set(tools_dir.to_str().unwrap().to_string());
    }

    let icons_dir = matches.get_one::<std::path::PathBuf>("icons-dir");
    if let Some(icons_dir) = icons_dir {
        Variable::IconsDirectory.set(icons_dir.to_str().unwrap().to_string());
    }

    let bin_dir = matches.get_one::<std::path::PathBuf>("bin-dir");
    if let Some(bin_dir) = bin_dir {
        Variable::BinariesDirectory.set(bin_dir.to_str().unwrap().to_string());
    }

    let verbose = matches.get_flag("verbose");
    if verbose {
        Variable::Verbose.set("true");
    }
}
