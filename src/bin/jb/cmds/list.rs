use clap::Command;
use console::style;
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
        style("Tool").bold().underlined(),
        style("Version").bold().underlined(),
        style("Build").bold().underlined(),
        style("Release Type").bold().underlined(),
    );

    let checkmark = style(char::from_u32(0x2714).unwrap()).green();
    let cross = style(char::from_u32(0x2718).unwrap()).red();

    for tool in &installed_tools {
        let linked = tool.is_linked();

        let icon = if linked {
            &checkmark
        } else {
            &cross
        };

        let kind = tool.kind.to_string();
        let version = if let Some(version) = &tool.version {
            version.to_string()
        } else {
            "N/A".to_string()
        };
        let build = if let Some(build) = &tool.build {
            build.to_string()
        } else {
            "N/A".to_string()
        };
        let release = if let Some(release) = &tool.release {
            release.to_string()
        } else {
            "N/A".to_string()
        };

        if linked {
            println!(
                "{:<1} {:<30} {:<15} {:<15} {:<15}",
                icon,
                kind,
                version,
                build,
                release,
            );
        } else {
            println!(
                "{:<1} {:<30} {:<15} {:<15} {:<15}",
                icon,
                style(kind).dim(),
                style(version).dim(),
                style(build).dim(),
                style(release).dim(),
            );
        }
    }

    if installed_tools.is_empty() {
        println!(
            "{}",
            style(
                format!(
                    "{:<1} {:<30} {:<15} {:<15} {:<15}",
                    " ",
                    "Empty",
                    "Empty",
                    "Empty",
                    "Empty",
                )
            ).italic().dim(),
        );

        tracing::warn!("No JetBrains tools installed");
    }

    Ok(())
}
