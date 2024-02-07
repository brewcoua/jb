//! Action module for tool.
//!
//! This module contains the different actions that can be performed on a tool.
//! They are automatically implemented when brought into scope.

mod list;
mod link;
mod probe;

pub use list::List;
pub use link::Link;
pub use probe::Probe;