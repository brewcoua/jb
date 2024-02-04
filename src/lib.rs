//! # jb-lib
//! This is a library crate for `JetBrains` tooling.

#[cfg(not(target_os = "linux"))]
compile_error!("This crate only supports Linux");

pub mod env;
pub mod error;
pub mod tool;
pub mod parse;

pub use error::{Batch, Result};
pub use tool::Tool;
