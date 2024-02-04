//! Action module for tool.
//!
//! This module contains the different actions that can be performed on a tool.
//! They are automatically implemented when brought into scope.

mod install;
mod list;
mod link;

pub use install::Install;
pub use list::List;
pub use link::Link;