use std::fmt::Write;
use std::thread;
use anyhow::{anyhow, Context};

use clap::{arg, value_parser, Command};
use console::Emoji;
use indicatif::{MultiProgress, ProgressBar};
use jb::{Tool, Result, Batch};
use jb::api::deserial::Download;
use jb::tool::{Install, Link, Probe};

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
            arg!(--clean)
                .help("Clean up old versions after installing")
                .required(false),
        )
}

static LOOKING_GLASS: Emoji = Emoji("🔎", "");
static LINK: Emoji = Emoji("🔗", "");
static DOWNLOAD: Emoji = Emoji("⬇️", "");
static CLEAN: Emoji = Emoji("🧹", "");
static CHECK: Emoji = Emoji("✅ ", "");

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tools: Vec<Tool> = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tools")
        .map(|tool| tool.clone())
        .collect();

    let clean = args.get_flag("clean");
    let mut error_batch = Batch::new();

    jb::info!("{} Resolving tool releases...", LOOKING_GLASS);

    // First step, find releases for all tools. If any fails, ignore them (while warning)
    let mut tools = crate::concurrent_step!(error_batch, tools, |mut tool: Tool| {
        jb::make!("{}", tool.as_str());

        let release = match tool.sync() {
            Ok(release) => release,
            Err(err) => {
                jb::warn!("Failed to fetch release for {tool}, skipping...");
                return Err(err);
            }
        };

        jb::debug!("Found release: {tool}");
        Ok((tool, release))
    });

    // Remove duplicate tools (avoid installing the same tool twice)
    tools.sort_by(|a, b| a.0.cmp(&b.0));
    tools.dedup_by(|a, b| a.0 == b.0);

    // Second step, download and extract all tools. If any fails, ignore them (while warning)
    // All errors will be collected and returned at the end
    jb::info!("{} Downloading tools...", DOWNLOAD);

    let m = MultiProgress::new();
    let ps = indicatif::ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");

    let mut tools = crate::concurrent_step!(error_batch, tools, |(tool, release): (Tool, Download)| {
        jb::make!("{}", tool.as_str());

        let install_dir = tool.as_path();

        let result = jb::util::download_extract(&release.link, &install_dir, Some(&pb))
            .with_context(|| format!("Failed to download {}", tool.as_str()));

        pb.finish();
        if let Err(e) = result {
            jb::warn!("Failed to download {}, skipping...", tool.as_str());

            // Make sure to delete the directory if it was created
            if install_dir.exists() {
                std::fs::remove_dir_all(&install_dir)
                    .with_context(|| format!("Failed to clean up {}", install_dir.display()))?;
            }

            return Err(e);
        }

        Ok(tool)
    }, {
        let pb = m.add(ProgressBar::new(100));
        pb.set_style(ps.clone());
    });

    m.clear().unwrap();

    // Third step, link all tools. If any fails, ignore them (while warning)
    jb::info!("{} Linking tools...", LINK);

    //* Remove all duplicate tool kinds, keeping the latest version (we can only have one version linked of each tool)
    let mut filtered_tools = tools.clone();
    filtered_tools.sort_by(|a, b| a.cmp(&b));
    filtered_tools.dedup_by(|a, b| a.kind == b.kind);

    crate::concurrent_step!(error_batch, filtered_tools, |tool: Tool| {
        jb::make!("{}", tool.as_str());

        let result = tool.link()
            .with_context(|| format!("Failed to link {}", tool.as_str()));

        if let Err(e) = result {
            jb::warn!("Failed to link {}, skipping...", tool.as_str());
            return Err(e);
        }

        Ok(())
    });

    // Fourth step, clean up old versions, if requested
    if clean {
        jb::info!("{} Cleaning up old versions...", CLEAN);

        tools = crate::concurrent_step!(error_batch, tools, |tool: Tool| {
            jb::make!("{}", tool.as_str());

            let result = tool.uninstall()
                .with_context(|| format!("Failed to clean {}", tool.as_str()));

            if let Err(e) = result {
                jb::warn!("Failed to clean {}, skipping...", tool.as_str());
                return Err(e);
            }

            Ok(tool)
        });
    }

    jb::info!("{} Done!", CHECK);
    for tool in tools {
        println!("- {tool}");
    }


    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}

