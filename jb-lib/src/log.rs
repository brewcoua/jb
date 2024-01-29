//! Logging utilities
//!
//! Used for logging in concurrent environments. Each thread can set its own task name, and log messages will be prefixed with the task name.

use colored::Colorize;
use log::Level;
use once_cell::sync::Lazy;

use std::sync::{Arc, Mutex};
use std::thread;

use super::env::Variable;

pub struct TaskLogger {
    tasks: Arc<Mutex<Vec<Task>>>,
    spans: Arc<Mutex<Vec<Span>>>,
}

struct Task {
    id: thread::ThreadId,
    name: String,
}

pub struct Span {
    id: usize,
    timestamp: u128,
}

impl TaskLogger {
    pub fn instance() -> &'static Self {
        static INSTANCE: Lazy<TaskLogger> = Lazy::new(|| TaskLogger::new());
        &INSTANCE
    }

    fn new() -> Self {
        let tasks = Arc::new(Mutex::new(vec![]));
        let spans = Arc::new(Mutex::new(vec![]));

        Self { tasks, spans }
    }

    pub fn set_task(&self, name: &str) {
        let mut tasks = self.tasks.lock().unwrap();
        let id = thread::current().id();

        // Remove any existing tasks for this thread
        tasks.retain(|t| t.id != id);

        tasks.push(Task {
            id,
            name: name.to_string(),
        });
    }

    pub fn start_span(&self) -> usize {
        let mut spans = self.spans.lock().unwrap();
        let id = spans.len();
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        spans.push(Span { id, timestamp });
        id
    }

    pub fn end_span(&self, id: usize) -> u128 {
        let mut spans = self.spans.lock().unwrap();
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        let span = spans
            .iter()
            .find(|s| s.id == id)
            .expect("Could not find span");

        let elapsed = now - span.timestamp;
        let id = span.id;

        spans.retain(|s| s.id != id);

        elapsed
    }

    pub fn log(&self, level: Level, content: &str, span: Option<usize>) {
        // Verify that log level is not filtered
        if self.is_filtered(level) {
            if let Some(span) = span {
                self.end_span(span);
            }
            return;
        }

        let mut tasks = self.tasks.lock().unwrap();
        let id = thread::current().id();
        let task = tasks
            .iter_mut()
            .find(|t| t.id == id);
        let task = match task {
            Some(task) => task,
            None => {
                tasks.push(Task {
                    id,
                    name: "unknown".to_string(),
                });
                tasks.last_mut().unwrap()
            }
        };

        let elapsed = match span {
            Some(span) => Some(self.end_span(span)),
            None => None,
        };

        match level {
            Level::Error => eprintln!("{}", task.format(&level, content, elapsed)),
            _ => println!("{}", task.format(&level, content, elapsed)),
        }
    }

    fn is_filtered(&self, level: Level) -> bool {
        let level = match level {
            Level::Error => "error",
            Level::Warn => "warn",
            Level::Info => "info",
            Level::Debug => "debug",
            Level::Trace => "trace",
        };

        let verbose = Variable::get::<String>(Variable::Verbose);
        if verbose == "true" || verbose == "1" {
            return false;
        }

        let level = level.to_string();
        let log_level = Variable::get::<String>(Variable::LogLevel);

        match log_level.as_str() {
            "off" => true,
            "error" => level != "error",
            "warn" => level != "error" && level != "warn",
            "info" => level != "error" && level != "warn" && level != "info",
            "debug" => level != "error" && level != "warn" && level != "info" && level != "debug",
            _ => false,
        }
    }
}

impl Task {
    pub fn format(&self, level: &Level, content: &str, elapsed: Option<u128>) -> String {
        let mut output = String::new();

        let level_name = match level {
            Level::Error => "ERROR".bright_red().bold(),
            Level::Warn => "WARN".bright_yellow().bold(),
            Level::Info => "INFO".bright_green().bold(),
            Level::Debug => "DEBUG".bright_blue().bold(),
            Level::Trace => "TRACE".bright_magenta().bold(),
        };

        output.push_str(&format!(
            "{}{:<5} {}{}",
            "[".dimmed().bold(),
            level_name,
            self.name.dimmed(),
            "]".dimmed().bold(),
        ));

        let content = match level {
            Level::Error => content.red(),
            Level::Warn => content.yellow(),
            Level::Info => content.green(),
            Level::Debug => content.blue(),
            Level::Trace => content.magenta(),
        };

        output.push_str(&format!(" {}", content));

        if let Some(elapsed) = elapsed {
            let duration = std::time::Duration::from_millis(elapsed.try_into().unwrap());
            let duration = humantime::format_duration(duration);
            output.push_str(&format!(" ({})", duration).dimmed().italic());
        }

        output
    }
}

#[macro_export]
macro_rules! set_task {
    ($name:expr) => {
        $crate::log::TaskLogger::instance().set_task($name);
    };
}

#[macro_export]
macro_rules! log {
    ($level:expr, $($arg:tt)*) => {
        $crate::log::TaskLogger::instance().log(
            $level,
            format!($($arg)*).as_str(),
            None,
        );
    };
}

#[macro_export]
macro_rules! log_span {
    (($span:expr) $level:expr, $($arg:tt)*) => {
        $crate::log::TaskLogger::instance().log(
            $level,
            format!($($arg)*).as_str(),
            Some($span),
        )
    };
    ($level:expr, $($arg:tt)*) => ({
        $crate::log::TaskLogger::instance().log(
            $level,
            format!($($arg)*).as_str(),
            None,
        );
        $crate::log::TaskLogger::instance().start_span()
    })
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        $crate::log!(log::Level::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! info_span {
    (($span:expr) $($arg:tt)*) => {
        $crate::log_span!(($span) log::Level::Info, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log_span!(log::Level::Info, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        $crate::log!(log::Level::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! debug_span {
    (($span:expr) $($arg:tt)*) => {
        $crate::log_span!(($span) log::Level::Debug, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log_span!(log::Level::Debug, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        $crate::log!(log::Level::Warn, $($arg)*)
    };
}

#[macro_export]
macro_rules! warn_span {
    (($span:expr) $($arg:tt)*) => {
        $crate::log_span!(($span) log::Level::Warn, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log_span!(log::Level::Warn, $($arg)*)
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::log!(log::Level::Error, $($arg)*);
    };
}

#[macro_export]
macro_rules! error_span {
    (($span:expr) $($arg:tt)*) => {
        $crate::log_span!(($span) log::Level::Error, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log_span!(log::Level::Error, $($arg)*)
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        $crate::log!(log::Level::Trace, $($arg)*)
    };
}

#[macro_export]
macro_rules! trace_span {
    (($span:expr) $($arg:tt)*) => {
        $crate::log_span!(($span) log::Level::Trace, $($arg)*)
    };
    ($($arg:tt)*) => {
        $crate::log_span!(log::Level::Trace, $($arg)*)
    };
}