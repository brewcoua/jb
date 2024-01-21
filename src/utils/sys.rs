use std::path::PathBuf;

pub fn mktemp_dir() -> Result<PathBuf, &'static str> {
    let output = std::process::Command::new("mktemp")
        .arg("-d")
        .output()
        .or_else(|_| {
            log::error!("Failed to create temporary directory");
            Err("Failed to create temporary directory")
        })?;

    if !output.status.success() {
        log::error!("Failed to create temporary directory");
        log::error!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to create temporary directory")
    } else {
        Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
    }
}

pub fn symlink(source: &PathBuf, destination: &PathBuf) -> Result<(), &'static str> {
    if destination.exists() {
        std::fs::remove_file(destination).or_else(|_| {
            log::error!("Failed to remove existing file {}", destination.display());
            Err("Failed to remove existing file")
        })?;
    }

    std::os::unix::fs::symlink(source, destination).or_else(|_| {
        log::error!("Failed to create symlink from {} to {}", source.display(), destination.display());
        Err("Failed to create symlink")
    })?;

    Ok(())
}

pub fn get_binary_dir() -> Result<PathBuf, &'static str> {
    let output = std::process::Command::new("systemd-path")
        .arg("user-binaries")
        .output()
        .expect("Failed to get binary directory");

    if !output.status.success() {
        log::error!("Failed to get binary directory");
        log::error!("{}", String::from_utf8_lossy(&output.stderr));
        return Err("Failed to get binary directory")
    }

    Ok(PathBuf::from(String::from_utf8_lossy(&output.stdout).trim()))
}