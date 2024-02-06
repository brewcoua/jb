use std::fmt::Display;
use std::thread::ThreadId;
use std::time::Instant;

use console::style;
use indicatif::HumanDuration;
use once_cell::sync::Lazy;
use dashmap::DashMap;
use dashmap::mapref::one::RefMut;
use crate::env::Variable;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug)]
pub struct Logger {
    pub name: Option<String>,
    pub started: Instant,
}

static LOGGERS: Lazy<DashMap<ThreadId, Logger>> = Lazy::new(|| DashMap::new());

impl Logger {
    /// Get the logger for the current thread or create a new one if it does not exist.
    ///
    /// This is a convenience method for getting the logger for the current thread.
    pub fn instance<'a>() -> RefMut<'a, ThreadId, Logger> {
        let id = std::thread::current().id();

        if let Some(logger) = LOGGERS.get_mut(&id) {
            logger
        } else {
            let logger = Logger {
                name: None,
                started: Instant::now(),
            };
            LOGGERS.insert(id, logger);
            LOGGERS.get_mut(&id).unwrap()
        }
    }

    /// Log a message.
    ///
    /// This logs a message to the console.
    /// It may be filtered based on the verbosity level.
    /// An elapsed time may be included in the message.
    pub fn log<T>(&self, level: Level, message: T, elapsed: bool)
        where
            T: Display,
    {
        let verbose = Variable::Verbose.get_bool();
        if !verbose && level == Level::Debug {
            return;
        }

        println!("{}", self.format(level, message, elapsed));
    }

    fn format<T>(&self, level: Level, message: T, elapsed: bool) -> String
        where
            T: Display,
    {
        let level = match level {
            Level::Debug => {
                style("DEBUG").cyan()
            }
            Level::Info => {
                style("INFO").green()
            }
            Level::Warning => {
                style("WARNING").yellow()
            }
            Level::Error => {
                style("ERROR").red()
            }
        };

        format!( // [ERROR name] message
                 "{}{}{}{} {}{}",
                 style("[").bold().dim(),
                 level,
                 if let Some(name) = &self.name {
                     format!(" {}", style(name).italic())
                 } else {
                     String::new()
                 },
                 style("]").bold().dim(),
                 message,
                 if elapsed {
                     format!(" ({} elapsed)", self.elapsed())
                 } else {
                     String::new()
                 }
        )
    }

    fn elapsed(&self) -> String {
        let elapsed = self.started.elapsed();
        format!("{}", HumanDuration(elapsed))
    }
}

#[macro_export]
macro_rules! make {
    ($($arg:tt)*) => {
        let mut logger = $crate::log::Logger::instance();
        logger.name = Some(format!($($arg)*));
        logger
    };
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::Logger::instance().log($level, format!($($arg)*), false)
    };
}
#[macro_export]
macro_rules! log_elapsed {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::Logger::instance().log($level, format!($($arg)*), true)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Info, $($arg)*)
    };
}
#[macro_export]
macro_rules! info_elapsed {
    ($($arg:tt)*) => {
        $crate::log_elapsed!($crate::log::Level::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Warning, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!($crate::log::Level::Error, $($arg)*)
    };
}
