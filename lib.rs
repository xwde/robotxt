#![forbid(unsafe_code)]

//!
//!
//! ## Examples
//!
//! - Parse the part of the `robots.txt` file related to
//! the specific user-agent.
//!
//! ```rust
//! use robotxt::Robots;
//! // let r = Robots::from_directives(vec![], "*");
//! ```
//!
//! - Parse the whole `robots.txt` file.
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
