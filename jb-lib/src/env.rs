//! Module for handling defaults and environment variables.

use std::env;
use std::path;

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    Verbose,
    ToolsDirectory,
}

impl Variable {
    /// Get the value of a variable.
    #[must_use]
    pub fn get<T>(var: Variable) -> T
    where
        T: From<String> + Send + Sync,
    {
        var.get_or(var.default::<T>())
    }

    /// Get the value of a variable, or a default value if it is not set.
    #[must_use]
    pub fn get_or<T>(&self, default: T) -> T
    where
        T: From<String> + Send + Sync,
    {
        let var = match self {
            Variable::Verbose => env::var("JB_VERBOSE"),
            Variable::ToolsDirectory => env::var(self.env()),
        };

        match var {
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
            Variable::ToolsDirectory => {
                path::PathBuf::from(env::var("HOME").expect("HOME environment variable not set"))
                    .join(".local/share/JetBrains")
                    .to_str()
                    .unwrap()
                    .to_string()
                    .into()
            }
        }
    }

    /// Get the name of the environment variable.
    #[must_use]
    pub fn env(&self) -> &'static str {
        match self {
            Variable::Verbose => "JB_VERBOSE",
            Variable::ToolsDirectory => "JB_TOOLS_DIR",
        }
    }
}
