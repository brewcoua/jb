use std::fmt::Write;
use std::sync::Arc;
use std::thread;
use anyhow::{anyhow, Context};

use clap::{arg, value_parser, Command};
use console::Emoji;
use indicatif::{MultiProgress, ProgressBar};
use jb::{Tool, Result, Batch};
use jb::tool::Probe;

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

// Emoji for showing the release fetch
static LOOKING_GLASS: Emoji = Emoji("🔍", "🔎");
static HAMMER: Emoji = Emoji("🔨", "⚒");

pub(crate) fn dispatch(args: &clap::ArgMatches) -> Result<()> {
    let tools = args
        .get_many::<Tool>("tools")
        .expect("Could not find argument tools");

    let _clean = Arc::new(args.get_flag("clean"));
    let mut error_batch = Batch::new();

    jb::info!("{} Resolving tool releases...", LOOKING_GLASS);

    // First step, find releases for all tools. If any fails, ignore them (while warning)
    let handles: Vec<_> = tools
        .map(|tool| {
            let mut tool = tool.clone();

            thread::spawn(move || {
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
            })
        }).collect();

    let mut tools = vec![];

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(Ok(result)) => tools.push(result),
            Ok(Err(e)) => error_batch.add(e),
            Err(e) => error_batch.add(anyhow!("Thread panicked: {:?}", e)),
        }
    }

    // Remove duplicate tools (avoid installing the same tool twice)
    tools.sort_by(|a, b| a.0.cmp(&b.0));
    tools.dedup_by(|a, b| a.0 == b.0);

    if !error_batch.is_empty() {
        jb::warn!("Failed to fetch releases for some tools");
        jb::warn!("{}", error_batch);
    }

    // Second step, download and extract all tools. If any fails, ignore them (while warning)
    // All errors will be collected and returned at the end

    jb::info!("{} Downloading tools...", HAMMER);

    let m = MultiProgress::new();
    let ps = indicatif::ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] {wide_bar:.cyan/blue} {bytes}/{total_bytes} ({eta})")
        .unwrap()
        .with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");
    let handles: Vec<_> = tools
        .iter()
        .map(|el| {
            let pb = m.add(ProgressBar::new(100));
            pb.set_style(ps.clone());

            let (tool, release) = el.clone();

            thread::spawn(move || {
                jb::make!("{}", tool.as_str());

                let install_dir = tool.as_path();

                let result = jb::util::download_extract(&release.link, &install_dir, Some(&pb))
                    .with_context(|| format!("Failed to install {}", tool.as_str()));

                if let Err(e) = result {
                    jb::warn!("Failed to install {}, skipping...", tool.as_str());
                    return Err(e);
                }

                pb.finish();
                Ok(tool)
            })
        }).collect();

    let mut tools = vec![];

    for handle in handles {
        let result = handle.join();
        match result {
            Ok(Ok(tool)) => tools.push(tool),
            Ok(Err(e)) => error_batch.add(e),
            Err(e) => error_batch.add(anyhow!("Thread panicked: {:?}", e)),
        }
    }
    m.clear().unwrap();

    if error_batch.is_empty() {
        Ok(())
    } else {
        Err(error_batch)
    }
}
