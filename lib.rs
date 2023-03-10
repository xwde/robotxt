#![forbid(unsafe_code)]

//! The implementation of the robots.txt protocol (or URL exclusion protocol)
//! with the support of `crawl-delay`, `sitemap`, and universal `*` match
//! extensions (according to the RFC specification).
//!
//! ## Examples
//!
//! - Parse the set of directives related to the specific `user-agent` in
//! the provided `robots.txt` file.
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

mod build;
pub use build::*;

mod fetch;
pub use fetch::*;

mod parse;
pub use parse::*;
