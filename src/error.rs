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

    /// Get the first error in the batch
    #[must_use]
    pub fn first(&self) -> Option<&Error> {
        self.errors.first()
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

    /// Extend the batch with another batch
    pub fn extend(&mut self, other: Batch) {
        self.errors.extend(other.errors);
    }
}

impl Display for Batch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.errors.is_empty() {
            return write!(f, "No errors occurred");
        } else if self.errors.len() == 1 {
            return write!(f, "{:?}", self.first().unwrap());
        }

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
