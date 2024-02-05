//! Module for handling errors in a batch
//!
//! This module provides a `Batch` struct that can be used to collect multiple errors that occur across multiple tasks.
//! This is useful when you want to collect all errors that occur during a command and then display them all at once.

use anyhow::Error;
use std::fmt::Display;

/// A type alias for a result that can return a batch of errors
pub type Result<T> = std::result::Result<T, Batch>;

/// A batch of errors that occurred while executing a command
pub struct Batch {
    errors: Vec<Error>,
}

impl Batch {
    /// Create a new batch of errors
    #[must_use]
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Create a new batch of errors from a single error
    #[must_use]
    pub fn from(error: Error) -> Self {
        let mut batch = Self::new();
        batch.add(error);
        batch
    }

    /// Add an error to the batch
    pub fn add(&mut self, error: Error) {
        self.errors.push(error);
    }

    /// Check if the batch is empty
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors in the batch
    #[must_use]
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Get the errors in the batch
    #[must_use]
    pub fn errors(&self) -> &Vec<Error> {
        &self.errors
    }
}

impl Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} error{} occurred:",
            self.errors.len(),
            if self.errors.len() == 1 { "" } else { "s" }
        )?;
        for error in &self.errors {
            writeln!(f, "{error:?}")?;
        }
        Ok(())
    }
}

impl Default for Batch {
    fn default() -> Self {
        Self::new()
    }
}

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