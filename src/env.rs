//! Module for handling defaults and environment variables.

use std::env;
use std::path::PathBuf;
use std::io::IsTerminal;
use nix::unistd::Uid;

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    /// Whether to enable verbose logging.
    ///
    /// `JB_VERBOSE`
    Verbose,
    /// Whether to enable notifications for long-running tasks.
    ///
    /// `JB_NOTIFY`
    Notify,
    /// The directory where tools are installed. (e.g. /usr/local/share/JetBrains/apps)
    /// The default value is $HOME/.local/share/JetBrains/apps or /usr/local/share/JetBrains/apps if running as root.
    ///
    /// `JB_TOOLS_DIR`
    ToolsDirectory,
    /// The directory where tool icons are installed. (e.g. /usr/local/share/JetBrains/icons)
    /// The default value is $HOME/.local/share/JetBrains/icons or /usr/local/share/JetBrains/icons if running as root.
    ///
    /// `JB_ICONS_DIR`
    IconsDirectory,
    /// The directory where tool binaries are installed. (e.g. /usr/local/bin)
    /// The default value is $HOME/.local/bin or /usr/local/bin if running as root.
    ///
    /// `JB_BINARIES_DIR`
    BinariesDirectory,
    /// The directory where desktop files are installed. (e.g. /usr/local/share/applications)
    /// The default value is $HOME/.local/share/applications or /usr/local/share/applications if running as root.
    ///
    /// `JB_DESKTOP_DIR`
    DesktopDirectory,

    /// The directory where systemd service files are installed. (e.g. /usr/local/share/systemd/user)
    /// This is used to manage timers for automatic updates.
    /// The default value is $HOME/.config/systemd/user or /etc/systemd/user if running as root.
    ///
    /// `JB_SYSTEMD_DIR`
    SystemdDirectory,
}

impl Variable {
    /// Get the value of the variable.
    #[must_use]
    pub fn get<T>(&self) -> T
    where
        T: From<String> + Send + Sync,
    {
        self.get_or(self.default::<T>())
    }

    /// Get the value of the variable as a bool.
    #[must_use]
    pub fn get_bool(&self) -> bool {
        matches!(self.get::<String>().as_str(), "true" | "1")
    }

    /// Set the value of the variable.
    pub fn set<T>(&self, value: T)
    where
        T: Into<String>,
    {
        env::set_var(self.env(), value.into());
    }

    /// Get the value of a variable.
    #[must_use]
    pub fn get_one<T>(var: Variable) -> T
    where
        T: From<String> + Send + Sync,
    {
        var.get_or(var.default::<T>())
    }

    /// Set the value of a variable.
    pub fn set_one<T>(var: Variable, value: T)
    where
        T: Into<String>,
    {
        env::set_var(var.env(), value.into());
    }

    /// Get the value of a variable, or a default value if it is not set.
    #[must_use]
    pub fn get_or<T>(&self, default: T) -> T
    where
        T: From<String> + Send + Sync,
    {
        match env::var(self.env()) {
            Ok(value) => value.into(),
            Err(_) => default,
        }
    }

    /// Get the default value for a variable.
    ///
    /// # Panics
    /// If the default value cannot be determined.
    /// For example, if the `HOME` environment variable is not set.
    #[must_use]
    pub fn default<T>(&self) -> T
    where
        T: From<String> + Send + Sync,
    {
        match self {
            Variable::Verbose => "false".to_string().into(),
            Variable::Notify => {
                // True if terminal is not interactive else false
                std::io::stdout().is_terminal()
                    .then(|| "false".to_string().into())
                    .unwrap_or("true".to_string().into())
            }
            Variable::ToolsDirectory => {
                if Self::is_root() {
                    return "/usr/local/share/JetBrains/apps".to_string().into();
                }

                PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".local/share/JetBrains/apps")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            },
            Variable::BinariesDirectory => {
                if Self::is_root() {
                    return "/usr/local/bin".to_string().into();
                }

                PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".local/bin")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            },
            Variable::IconsDirectory => {
                if Self::is_root() {
                    return "/usr/local/share/JetBrains/icons".to_string().into();
                }

                PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".local/share/JetBrains/icons")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            },
            Variable::DesktopDirectory => {
                if Self::is_root() {
                    return "/usr/local/share/applications".to_string().into();
                }

                PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".local/share/applications")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            },
            Variable::SystemdDirectory => {
                if Self::is_root() {
                    return "/etc/systemd/user".to_string().into();
                }

                PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".config/systemd/user")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            },
        }
    }

    /// Get the name of the environment variable.
    #[must_use]
    pub fn env(&self) -> &'static str {
        match self {
            Variable::Verbose => "JB_VERBOSE",
            Variable::Notify => "JB_NOTIFY",
            Variable::ToolsDirectory => "JB_TOOLS_DIR",
            Variable::IconsDirectory => "JB_ICONS_DIR",
            Variable::BinariesDirectory => "JB_BINARIES_DIR",
            Variable::DesktopDirectory => "JB_DESKTOP_DIR",
            Variable::SystemdDirectory => "JB_SYSTEMD_DIR",
        }
    }

    fn is_root() -> bool {
        Uid::effective().is_root()
    }
}
