use std::fmt::Write;
use anyhow::Context;
use jb::tool::{Link, Probe};
use crate::emoji::*;

pub(crate) fn install_tools(error_batch: &mut jb::Batch, tools: Vec<jb::Tool>, force: bool) -> Vec<jb::Tool> {
    jb::info!("{LOOKING_GLASS} Resolving tool releases...");

    // First step, find releases for all tools. If any fails, ignore them (while warning)
    let mut tools = crate::concurrent_step!(error_batch, tools, |mut tool: jb::Tool| {
        jb::make!("{}", tool.as_str());

        let release = match tool.sync() {
            Ok(release) => release,
            Err(err) => {
                jb::warn!("Failed to fetch release for {tool}, skipping... {SKIP}");
                return Err(err);
            }
        };

        jb::debug!("Found release: {tool}");
        Ok((tool, release))
    });

    if tools.is_empty() {
        return Vec::new();
    }

    tools.sort_by(|a, b| a.0.cmp(&b.0));
    tools.dedup_by(|a, b| a.0 == b.0);

    let tools: Vec<(jb::Tool, jb::api::deserial::Download)> = tools.iter().filter(|(tool, _)| {
        if tool.as_path().exists() {
            if force {
                jb::warn!("{tool} is already installed, but force is enabled, continuing...");
            } else {
                jb::warn!("{tool} is already installed, skipping... {SKIP}");
                return false;
            }
        }
        true
    }).map(|(tool, release)| (tool.clone(), release.clone())).collect();

    if tools.is_empty() {
        return Vec::new();
    }

    jb::info!("{DOWNLOAD} Downloading tools...");

    let m = indicatif::MultiProgress::new();
    let ps = indicatif::ProgressStyle::with_template("{prefix:.bold.dim} [{elapsed_precise}] {wide_bar:.cyan/blue} {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
        .unwrap()
        .with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap())
        .progress_chars("#>-");

    let tools = crate::concurrent_step!(error_batch, tools, |(tool, release): (jb::Tool, jb::api::deserial::Download)| {
        jb::make!("{}", tool.as_str());

        pb.set_prefix(format!("[{}]", tool.as_str()));

        let install_dir = tool.as_path();

        // Make sure to pre-delete the directory if it exists
        if install_dir.exists() {
            std::fs::remove_dir_all(&install_dir)
                .with_context(|| format!("Failed to clean up {}", install_dir.display()))?;
        }

        let result = jb::util::download_extract(&release.link, &install_dir, Some(&release.checksum_link), Some(&pb))
            .with_context(|| format!("Failed to download {}", tool.as_str()));

        pb.finish();
        if let Err(e) = result {
            jb::warn!("Failed to download {}, skipping... {SKIP}", tool.as_str());

            // Make sure to delete the directory if it was created
            if install_dir.exists() {
                std::fs::remove_dir_all(&install_dir)
                    .with_context(|| format!("Failed to clean up {}", install_dir.display()))?;
            }

            return Err(e);
        }

        Ok(tool)
    }, {
        let pb = m.add(indicatif::ProgressBar::new(100));
        pb.set_style(ps.clone());
    });

    m.clear().unwrap();

    if tools.is_empty() {
        return tools;
    }

    // Third step, link all tools. If any fails, ignore them (while warning)
    jb::info!("{LINK} Linking tools...");

    //* Remove all duplicate tool kinds, keeping the latest version (we can only have one version linked of each tool)
    let mut filtered_tools = tools.clone();
    filtered_tools.sort();
    filtered_tools.dedup_by(|a, b| a.kind == b.kind);

    crate::concurrent_step!(error_batch, filtered_tools, |tool: jb::Tool| {
        jb::make!("{}", tool.as_str());

        let result = tool.link()
            .with_context(|| format!("Failed to link {}", tool.as_str()));

        if let Err(e) = result {
            jb::warn!("Failed to link {}, skipping... {SKIP}", tool.as_str());
            return Err(e);
        }

        Ok(())
    });

    tools
}

#[macro_export]
macro_rules! concurrent_step {
    ($error_batch:expr, $inputs:expr, $step:expr) => {{
        let handles: Vec<_> = $inputs
            .into_iter()
            .map(|input| {
                std::thread::spawn(|| $step(input))
            })
            .collect();

        let mut results: Vec<_> = vec![];

        for handle in handles {
            let result = handle.join();
            match result {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => $error_batch.add(e),
                Err(e) => $error_batch.add(anyhow::anyhow!("Thread panicked: {:?}", e)),
            }
        }

        results
    }};
    ($error_batch:expr, $inputs:expr, $step:expr, { $($prefunc:tt)* }) => {{
        let handles: Vec<_> = $inputs
            .into_iter()
            .map(|input| {
                // Run the prefunc
                $($prefunc)*

                std::thread::spawn(move || $step(input))
            })
            .collect();

        let mut results: Vec<_> = vec![];

        for handle in handles {
            let result = handle.join();
            match result {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => $error_batch.add(e),
                Err(e) => $error_batch.add(anyhow::anyhow!("Thread panicked: {:?}", e)),
            }
        }

        results
    }};
}