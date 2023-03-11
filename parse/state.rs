use nom::AsBytes;
use std::cmp::min;
use std::io::{BufReader, Error as IoError, Read};
use std::time::Duration;

use url::Url;

use crate::parse::{into_directives, Directive, Rule, Rules, BYTES_LIMIT};

///
fn try_agent(u: &[u8]) -> Option<String> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = u.trim().to_lowercase();
    Some(u)
}

///
fn try_sitemaps(u: &[u8]) -> Option<Url> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = Url::parse(u.as_str()).ok()?;
    Some(u)
}

///
fn try_rule(u: &[u8], allow: bool) -> Option<Rule> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = Rule::new(u.as_str(), allow).ok()?;
    Some(u)
}

///
fn try_delay(u: &[u8]) -> Option<Duration> {
    let u = String::from_utf8(u.to_vec()).ok()?;
    let u = u.parse::<f32>().ok()?;
    let u = Duration::try_from_secs_f32(u).ok()?;
    Some(u)
}

/// The `AccessResult` enum represents the result of the
/// `robots.txt` retrieval attempt. See [Robots::from_access].
#[derive(Debug)]
pub enum AccessResult<'a> {
    /// The `robots.txt` file was provided by the server and
    /// ready to be parsed.
    Successful(&'a [u8]),
    /// The `robots.txt` file has not been reached after
    /// at least five redirect hops. Treated as `Unavailable`.
    Redirect,
    /// The valid `robots.txt` file does not exist.
    /// The `Robots` assumes that there are no restrictions.
    /// The site is fully allowed.
    Unavailable,
    /// The `robots.txt` file could not be served.
    /// The site is fully disallowed.
    Unreachable,
}

const DEFAULT: &str = "*";

/// The `Robots` struct represents the set of directives related to
/// the specific `user-agent` in the provided `robots.txt` file.
#[derive(Debug, Clone)]
pub struct Robots {
    user_agent: String,
    always_rule: Option<bool>,
    rules: Rules,
    sitemaps: Vec<Url>,
}

impl Robots {
    /// Creates a new `Robots` from the always rule.
    pub fn from_always(always: bool, ua: &str) -> Self {
        let ua = ua.trim().to_lowercase();
        Self {
            user_agent: ua,
            always_rule: Some(always),
            rules: Rules::new(vec![], None),
            sitemaps: vec![],
        }
    }

    /// Creates a new `Robots` from the `AccessResult`.
    pub fn from_access(access: AccessResult, ua: &str) -> Self {
        use AccessResult::*;
        match access {
            Successful(r) => Self::from_slice(r, ua),
            Redirect | Unavailable => Self::from_always(true, ua),
            Unreachable => Self::from_always(false, ua),
        }
    }

    /// Finds the longest matching user-agent and
    /// if the parser should check non-assigned rules.
    fn find_agent(directives: &[Directive], ua: &str) -> (String, bool) {
        // Collects all uas.
        let uas = directives.iter().filter_map(|ua2| match ua2 {
            Directive::UserAgent(ua2) => String::from_utf8(ua2.to_vec()).ok(),
            _ => None,
        });

        // Filters out non-acceptable uas.
        let ua = ua.trim().to_lowercase();
        let uas = uas.map(|ua2| ua2.trim().to_lowercase());
        let uas = uas.filter(|ua2| ua.starts_with(ua2.as_str()));

        // Finds the longest ua in the acceptable pool.
        let uas = uas.max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()));
        let uas = uas.unwrap_or(DEFAULT.to_string());

        // Determines if it should check non-assigned rules.
        let default = uas.eq(DEFAULT);
        (uas, default)
    }

    /// Creates a new `Robots` from the directives.
    fn from_directives(directives: &[Directive], ua: &str) -> Self {
        let (ua, mut captures_rules) = Self::find_agent(directives, ua);
        let mut captures_group = false;

        let mut rules = Vec::new();
        let mut delay = None;
        let mut maps = Vec::new();

        for directive in directives {
            match directive {
                Directive::UserAgent(u) => {
                    if let Some(u) = try_agent(u) {
                        if !captures_group || !captures_rules {
                            captures_rules = u.eq(&ua);
                        }
                    }

                    captures_group = true;
                    continue;
                }

                Directive::Sitemap(u) => {
                    if let Some(u) = try_sitemaps(u) {
                        maps.push(u);
                    }

                    continue;
                }

                Directive::Unknown(_) => continue,
                _ => captures_group = false,
            }

            if !captures_rules {
                continue;
            }

            match directive {
                Directive::Allow(u) | Directive::Disallow(u) => {
                    let allow = matches!(directive, Directive::Allow(_));
                    if let Some(u) = try_rule(u, allow) {
                        rules.push(u)
                    }
                }

                Directive::CrawlDelay(u) => {
                    if let Some(u) = try_delay(u) {
                        delay = delay.map(|c| min(c, u)).or(Some(u));
                    }
                }

                _ => unreachable!(),
            }
        }

        Self {
            user_agent: ua,
            always_rule: None,
            rules: Rules::new(rules, delay),
            sitemaps: maps,
        }
    }

    /// Creates a new `Robots` from the byte slice.
    pub fn from_slice(robots: &[u8], ua: &str) -> Self {
        // Limits the input to 500 kibibytes.
        let limit = min(robots.len(), BYTES_LIMIT);
        let robots = &robots[0..limit];

        // Replaces '\x00' with '\n'.
        let robots = robots.iter().map(|u| match u {
            b'\x00' => b'\n',
            v => *v,
        });

        let robots: Vec<_> = robots.collect();
        let robots = robots.as_bytes();

        let directives = into_directives(robots);
        Self::from_directives(directives.as_slice(), ua)
    }

    /// Creates a new `Robots` from the string slice.
    pub fn from_string(robots: &str, ua: &str) -> Self {
        let robots = robots.as_bytes();
        Self::from_slice(robots, ua)
    }

    /// Creates a new `Robots` from the generic reader.
    pub fn from_reader<R: Read>(reader: R, ua: &str) -> Result<Self, IoError> {
        let reader = reader.take(BYTES_LIMIT as u64);
        let mut reader = BufReader::new(reader);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let robots = buffer.as_slice();
        Ok(Self::from_slice(robots, ua))
    }
}

impl Robots {
    /// Returns the longest matching user-agent.
    pub fn user_agent(&self) -> String {
        self.user_agent.clone()
    }

    /// Returns true if the path is allowed for the longest matching user-agent.
    /// NOTE: Expects relative path.
    pub fn is_match(&self, path: &str) -> bool {
        match self.is_always() {
            Some(always) => always,
            None => self.rules.is_match(path),
        }
    }

    /// Returns `Some(_)` if the site is fully allowed or fully disallowed.
    pub fn is_always(&self) -> Option<bool> {
        match self.always_rule {
            Some(always) => Some(always),
            None if self.rules.is_empty() => Some(true),
            None => None,
        }
    }

    /// Returns the crawl-delay of the longest matching user-agent.
    pub fn delay(&self) -> Option<Duration> {
        self.rules.delay()
    }

    /// Returns all sitemaps.
    pub fn sitemaps(&self) -> Vec<Url> {
        self.sitemaps.clone()
    }
}

#[cfg(test)]
mod precedence {
    use super::*;

    static DIRECTIVES: &[Directive] = &[
        Directive::UserAgent(b"bot-robotxt"),
        Directive::Allow(b"/1"),
        Directive::Disallow(b"/"),
        Directive::UserAgent(b"*"),
        Directive::Allow(b"/2"),
        Directive::Disallow(b"/"),
        Directive::UserAgent(b"bot"),
        Directive::Allow(b"/3"),
        Directive::Disallow(b"/"),
    ];

    #[test]
    fn specific() {
        let r = Robots::from_directives(DIRECTIVES, "bot-robotxt");

        // Matches:
        assert!(r.is_match("/1"));

        // Doesn't match:
        assert!(!r.is_match("/2"));
        assert!(!r.is_match("/3"));
    }

    #[test]
    fn strict() {
        let r = Robots::from_directives(DIRECTIVES, "bot");

        // Matches:
        assert!(r.is_match("/3"));

        // Doesn't match:
        assert!(!r.is_match("/1"));
        assert!(!r.is_match("/2"));
    }

    #[test]
    fn missing() {
        let r = Robots::from_directives(DIRECTIVES, "super-bot");

        // Matches:
        assert!(r.is_match("/2"));

        // Doesn't match:
        assert!(!r.is_match("/1"));
        assert!(!r.is_match("/3"));
    }

    #[test]
    fn partial() {
        let r = Robots::from_directives(DIRECTIVES, "bot-super");

        // Matches:
        assert!(r.is_match("/3"));

        // Doesn't match:
        assert!(!r.is_match("/1"));
        assert!(!r.is_match("/2"));
    }
}
