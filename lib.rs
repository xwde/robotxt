#![forbid(unsafe_code)]

//! The implementation of the robots.txt protocol (or URL exclusion protocol)
//! with the support of `crawl-delay`, `sitemap`, `host` and universal `*`
//! match extensions (according to the RFC specification).
//!
//! ## Examples
//!
//! - Parse the set of directives of the provided `robots.txt`
//! file related to the specific user-agent.
//!
//! ```rust
//! use robotxt::Robots;
//!
//! let txt = r#"
//!     User-Agent: foobot
//!     Allow: /example/
//!     Disallow: /example/nope.txt
//! "#;
//!
//! let r = Robots::from_string(txt, "foobot");
//! assert!(r.is_match("/example/yeah.txt"));
//! assert!(!r.is_match("/example/nope.txt"));
//! ```
//!
//! - Parse all directives of the provided `robots.txt` file.
//!
//! ```rust
//! use robotxt::RobotsFile;
//!
//! let txt = r#"
//!     User-Agent: foobot
//!     Allow: /example/
//!     Disallow: /example/nope.txt
//! "#;
//!
//! let r = RobotsFile::from_string(txt);
//! assert!(r.is_match("/example/yeah.txt", "foobot"));
//! assert!(!r.is_match("/example/nope.txt", "foobot"));
//! ```

mod build;
mod internal;
mod parse;

pub use parse::*;
