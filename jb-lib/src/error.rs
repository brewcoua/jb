use anyhow::Error;
use std::fmt::Display;

pub type Result<T> = std::result::Result<T, Batch>;

/// A batch of errors that occurred while executing a command
pub struct Batch {
    errors: Vec<Error>,
}

impl Batch {
    /// Create a new batch of errors
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    /// Create a new batch of errors from a single error
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
    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    /// Get the number of errors in the batch
    pub fn len(&self) -> usize {
        self.errors.len()
    }

    /// Get the errors in the batch
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
            writeln!(f, "{:?}", error)?;
        }
        Ok(())
    }
}
