#![forbid(unsafe_code)]

//!
//!
//! ```rust
//! use robotxt::Robots;
//! // let r = Robots::from_directives(vec![], "*");
//! ```
//!
//! ```rust
//! use robotxt::RobotsFile;
//! // let r = RobotsFile::from_directives(vec![]);
//! // r.
//! ```

mod build;
mod parse;
mod state;

pub use state::*;
