use clap::Command;
use colored::Colorize;
use jb::{Tool, Result, bail_with};
use jb::tool::{Link, List};

pub(crate) fn command() -> Command {
    Command::new("list")
        .about("List installed JetBrains tools")
}

pub(crate) fn dispatch() -> Result<()> {
    let installed_tools = match Tool::list() {
        Ok(tools) => tools,
        Err(err) => bail_with!(err, "Failed to list tools"),
    };

    println!(
        "{:<1} {:<30} {:<15} {:<15} {:<15}",
        " ",
        "Tool".bold().underline(),
        "Version".bold().underline(),
        "Build".bold().underline(),
        "Release Type".bold().underline(),
    );

    let checkmark = char::from_u32(0x2714).unwrap().to_string();
    let cross = char::from_u32(0x2718).unwrap().to_string();

    for tool in &installed_tools {
        let linked = tool.is_linked();

        let icon = if linked {
            checkmark.green()
        } else {
            cross.red()
        };

        let line = format!(
            "{:<1} {:<30} {:<15} {:<15} {:<15}",
            icon,
            tool.kind.to_string(),
            if let Some(version) = &tool.version {
                version.to_string()
            } else {
                "N/A".to_string()
            },
            if let Some(build) = &tool.build {
                build.to_string()
            } else {
                "N/A".to_string()
            },
            if let Some(release) = &tool.release {
                release.to_string()
            } else {
                "N/A".to_string()
            },
        );

        if linked {
            println!("{line}");
        } else {
            println!(
                "{}",
                line.dimmed(),
            );
        }
    }

    if installed_tools.is_empty() {
        println!(
            "{}",
            format!(
                "{:<1} {:<30} {:<15} {:<15} {:<15}",
                " ",
                "Empty",
                "Empty",
                "Empty",
                "Empty",
            ).italic().dimmed(),
        );

        tracing::warn!("No JetBrains tools installed");
    }

    Ok(())
}
