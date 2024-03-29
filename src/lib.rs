//! # `JetBrains` CLI
//!
//! This crate is made to provide a simple CLI for `JetBrains` tools.

#[cfg(not(target_os = "linux"))]
compile_error!("This crate only supports Linux");

pub mod env;
pub mod log;
pub mod error;
pub mod tool;
pub mod api;
pub mod util;

pub use error::{Batch, Result};
pub use tool::Tool;
pub use util::notify;

/// Create a new batch of errors from a single error
#[macro_export]
macro_rules! batch {
    ($($arg:tt)*) => {
        $crate::error::Batch::from(anyhow::anyhow!($($arg)*))
    };
}

/// Add an error to the given batch
#[macro_export]
macro_rules! batch_with {
    ($batch:expr, $($arg:tt)*) => {
        $batch.add(anyhow::anyhow!($($arg)*))
    };
}

/// Return a batch of errors directly from a format string
#[macro_export]
macro_rules! bail {
    ($($arg:tt)*) => {
        return Err($crate::error::Batch::from(anyhow::anyhow!($($arg)*)))
    };
}

/// Return a batch of errors directly from an error with a formatted context
#[macro_export]
macro_rules! bail_with {
    ($err:expr, $($arg:tt)*) => {
        return Err($crate::error::Batch::from($err.context(format!($($arg)*))))
    };
}

/// Return a batch of errors directly from a result
#[macro_export]
macro_rules! catch {
    ($val:expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => $crate::bail!(err),
        }
    };
    ($val:expr, $ctx:expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => $crate::bail_with!(err, $ctx),
        }
    };
}

/// Return a batch of errors directly from a result, adding the error to the given batch
#[macro_export]
macro_rules! catch_with {
    ($batch:expr, $val:expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => {
                $crate::batch_with!($batch, err);
                return Err($batch);
            }
        }
    };
    ($batch:expr, $val:expr, $ctx:expr) => {
        match $val {
            Ok(val) => val,
            Err(err) => {
                $crate::batch_with!($batch, err.context($ctx));
                return Err($batch);
            }
        }
    };
}