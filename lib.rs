#![forbid(unsafe_code)]

//! The implementation of the robots.txt protocol (or URL exclusion protocol)
//! with the support of `crawl-delay`, `sitemap`, and universal `*` match
//! extensions (according to the RFC specification).
//!
//! ## Examples
//!
//! - parse the `user-agent` in the provided `robots.txt` file:
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
//! - build the new `robots.txt` file from provided directives:
//!
//! ```rust
//! use robotxt::Robots;
//! ```
//!
//! ## Links
//!
//! - [Request for Comments: 9309](https://www.rfc-editor.org/rfc/rfc9309.txt) on
//!   RFC-Editor.com
//! - [Introduction to Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/intro)
//!   on Google.com
//! - [How Google interprets Robots.txt](https://developers.google.com/search/docs/crawling-indexing/robots/robots_txt)
//!   on Google.com
//! - [What is Robots.txt file](https://moz.com/learn/seo/robotstxt) on Moz.com
//!
//! ## Notes
//!
//! The parser is based on
//! [Smerity/texting_robots](https://github.com/Smerity/texting_robots).
//!

mod build;
pub use build::*;

mod parse;
pub use parse::*;
