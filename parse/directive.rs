use std::fmt::{Debug, Formatter, Result as FmtResult};

use bstr::ByteSlice;

// TODO attach position
/// The `Directive` enum represents every supported `robots.txt` directive.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Directive<'a> {
    UserAgent(&'a [u8]),
    Allow(&'a [u8]),
    Disallow(&'a [u8]),
    CrawlDelay(&'a [u8]),
    Sitemap(&'a [u8]),
    Unknown(&'a [u8]),
}

impl Debug for Directive<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::UserAgent(v) => f.debug_tuple("UserAgent").field(&v.as_bstr()).finish(),
            Self::Allow(v) => f.debug_tuple("Allow").field(&v.as_bstr()).finish(),
            Self::Disallow(v) => f.debug_tuple("Disallow").field(&v.as_bstr()).finish(),
            Self::CrawlDelay(v) => f.debug_tuple("Crawl-delay").field(&v.as_bstr()).finish(),
            Self::Sitemap(v) => f.debug_tuple("Sitemap").field(&v.as_bstr()).finish(),
            Self::Unknown(v) => f.debug_tuple("Unknown").field(&v.as_bstr()).finish(),
        }
    }
}
