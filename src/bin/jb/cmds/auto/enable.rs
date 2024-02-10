use clap::Command;
use crate::emoji::CHECK;

pub(super) fn command() -> Command {
    Command::new("enable")
        .about("Enable automatic updates for tools")
}

pub(super) fn dispatch() -> jb::Result<()> {
    let systemd_path = jb::env::Variable::SystemdDirectory.get::<std::path::PathBuf>();

    let service_path = systemd_path.join("jb.service");
    let timer_path = systemd_path.join("jb.timer");

    if !service_path.exists() || !timer_path.exists() {
        jb::warn!("Services not found. Please run `jb auto set` first");
        return Ok(())
    }

    jb::debug!(
        "Enabling automatic updates with service: {} and timer: {}",
        service_path.display(),
        timer_path.display(),
    );

    jb::catch!(
        std::process::Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg("jb.service")
            .status()
    );

    jb::catch!(
        std::process::Command::new("systemctl")
            .arg("--user")
            .arg("enable")
            .arg("jb.timer")
            .status()
    );

    jb::catch!(
        std::process::Command::new("systemctl")
            .arg("--user")
            .arg("start")
            .arg("jb.timer")
            .status()
    );

    jb::info!("{CHECK} Automatic updates enabled");

    Ok(())
}