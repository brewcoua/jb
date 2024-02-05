use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::time::Instant;

use console::style;
use indicatif::{HumanDuration};

pub enum Level {
    Debug,
    Info,
    Warning,
    Error,
}

pub struct Logger {
    started: Instant,
    step: usize,
    max_step: usize,
}

impl Logger {
    pub fn instance() -> Arc<Mutex<Self>> {
        let logger = once_cell::sync::Lazy::new(|| {
            Arc::new(Mutex::new(Self::init(0)))
        });

        Arc::clone(&logger)
    }

    pub fn init(max_step: usize) {
        let mut logger = Logger::instance().lock().unwrap();
        logger.max_step = max_step;
    }

    pub fn log<T>(level: Level, message: T)
    where
        T: Display,
    {
        let mut logger = Self::instance().lock().unwrap();

        let level = match level {
            Level::Debug => {
                style("DEBUG").cyan()
            }
            Level::Info => {
                logger.step += 1;
                style("INFO").green()
            }
            Level::Warning => {
                style("WARNING").yellow()
            }
            Level::Error => {
                style("ERROR").red()
            }
        };

        println!(
            "{}{} {} {}",
            style("[").bold().dim(),
            level,
            style(format!("{}/{}]", logger.step, logger.max_step)).bold().dim(),
            message
        );
    }

    pub fn elapsed(&self) -> String {
        let elapsed = self.started.elapsed();
        format!("{}", HumanDuration(elapsed))
    }
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::Logger::instance().lock().unwrap().log($level, format!($($arg)*))
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!(Level::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!(Level::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!(Level::Warning, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!(Level::Error, $($arg)*)
    };
}
