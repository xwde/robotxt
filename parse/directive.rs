use std::fmt::{Debug, Formatter, Result as FmtResult};
use std::time::Duration;

use bstr::ByteSlice;
use url::Url;

use crate::state::Rule;

// TODO attach position
/// The `Directive` enum represents every
/// supported `robots.txt` directive.
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

impl Directive<'_> {
    ///
    pub fn try_agent(&self) -> Option<(String, &[u8])> {
        match &self {
            Directive::UserAgent(u) => {
                let str = String::from_utf8(u.to_vec()).ok()?;
                Some((str, u))
            }
            _ => None,
        }
    }

    ///
    pub fn try_rule(&self) -> Option<Rule> {
        match &self {
            Directive::Allow(u) => {
                let u = String::from_utf8(u.to_vec()).ok()?;
                let u = Rule::new(u.as_str(), true).ok()?;
                Some(u)
            }
            Directive::Disallow(u) => {
                let u = String::from_utf8(u.to_vec()).ok()?;
                let u = Rule::new(u.as_str(), false).ok()?;
                Some(u)
            }
            _ => None,
        }
    }

    ///
    pub fn try_sitemap(&self) -> Option<Url> {
        match &self {
            Directive::Sitemap(u) => {
                let u = String::from_utf8(u.to_vec()).ok()?;
                let u = Url::parse(u.as_str()).ok()?;
                Some(u)
            }
            _ => None,
        }
    }

    ///
    pub fn try_delay(&self) -> Option<Duration> {
        match &self {
            Directive::CrawlDelay(u) => {
                let u = String::from_utf8(u.to_vec()).ok()?;
                let u = u.parse::<f32>().ok()?;
                let u = Duration::try_from_secs_f32(u).ok()?;
                Some(u)
            }
            _ => None,
        }
    }
}
