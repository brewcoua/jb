//! # jb-lib
//! This is a library crate for `JetBrains` tooling.

pub mod log;
mod env;
pub mod error;
pub mod tool;
mod util;


pub use error::Result;
pub use tool::Tool;
