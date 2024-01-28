//! Module for handling defaults and environment variables.

use std::env;
use std::path;

#[derive(Debug, Clone, Copy)]
pub enum Variable {
    Verbose,
    LogLevel,
    ToolsDirectory,
}

impl Variable {
    pub fn get<T>(var: Variable) -> T
    where
        T: From<String> + Send + Sync,
    {
        var.get_or(var.default::<T>()).into()
    }

    pub fn get_or<T>(&self, default: T) -> T
    where
        T: From<String> + Send + Sync,
    {
        let var = match self {
            Variable::Verbose => env::var(self.env()),
            Variable::LogLevel => env::var(self.env()),
            Variable::ToolsDirectory => env::var(self.env()),
        };

        match var {
            Ok(value) => value.into(),
            Err(_) => default.into(),
        }
    }

    pub fn default<T>(&self) -> T
    where
        T: From<String> + Send + Sync,
    {
        match self {
            Variable::Verbose => "false".to_string().into(),
            Variable::LogLevel => "info".to_string().into(),
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

    pub fn env(&self) -> &'static str {
        match self {
            Variable::Verbose => "JB_VERBOSE",
            Variable::LogLevel => "JB_LOG",
            Variable::ToolsDirectory => "JB_TOOLS_DIR",
        }
    }
}
