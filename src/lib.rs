use clap::Parser;

mod tools;
mod arch;

pub fn dispatch(args: JetBrainsCLI) {
    match args {
        JetBrainsCLI::Install(_args) => {
            log::debug!("Fetching latest release for {}", _args.tool.as_code());
            let release = _args.tool.latest_release(None, None);
            log::debug!("Found latest release {}", release.version);
            let download = release.download(None, None);
            if let Some(download) = download {
                log::debug!("Found download for {}", _args.tool.as_code());
                log::debug!("Downloading {} from {}", _args.tool.as_code(), download.link);
                let response = reqwest::blocking::get(&download.link).expect("Failed to download");
                log::debug!("Downloaded {} from {}", _args.tool.as_code(), download.link);
                let bytes = response.bytes().expect("Failed to get bytes");
                log::debug!("Got bytes for {}", _args.tool.as_code());
                let mut file = std::fs::File::create(format!("{}.tar.gz", _args.tool.as_code())).expect("Failed to create file");
                log::debug!("Created file for {}", _args.tool.as_code());
                std::io::copy(&mut std::io::Cursor::new(bytes), &mut file).expect("Failed to copy bytes to file");
                log::debug!("Copied bytes to file for {}", _args.tool.as_code());
            } else {
                log::error!("Failed to find compatible download for {}", _args.tool.as_code());
            }
        }
    }
}

#[derive(Parser)]
#[command(name = "jb-cli")]
#[command(bin_name = "jb-cli")]
pub enum JetBrainsCLI {
    Install(InstallArgs),
}

#[derive(clap::Args)]
#[command(version, about = "Install any of JetBrains' IDEs and tools")]
pub struct InstallArgs {
    #[arg(long, short, action)]
    verbose: bool,
    #[arg(long, short, action)]
    force: bool,
    #[arg(long, short)]
    directory: Option<String>,
    #[arg()]
    tool: tools::Tool,
}