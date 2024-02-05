//! # jb-lib
//! This is a library crate for `JetBrains` tooling.

#[cfg(not(target_os = "linux"))]
compile_error!("This crate only supports Linux");

pub mod env;
pub mod error;
pub mod tool;
pub mod api;

pub use error::{Batch, Result};
pub use tool::Tool;

/// Create a new batch of errors from a single error
#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        $crate::error::Batch::from(anyhow::anyhow!($($arg)*))
    };
}

/// Add an error to the given batch
#[macro_export]
macro_rules! error_with {
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