use clap::Command;

pub(super) fn command() -> Command {
    Command::new("start")
        .about("Force start automatic updates for tools")
}

pub(super) fn dispatch() -> jb::Result<()> {
    let systemd_path = jb::env::Variable::SystemdDirectory.get::<std::path::PathBuf>();

    let service_path = systemd_path.join("jb.service");
    let timer_path = systemd_path.join("jb.timer");

    if !service_path.exists() || !timer_path.exists() {
        jb::info!("Automatic updates are not enabled");
        return Ok(())
    }

    jb::catch!(
        std::process::Command::new("systemctl")
            .arg("start")
            .arg("--user")
            .arg("jb.service")
            .status()
    );

    Ok(())
}