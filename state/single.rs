use std::io::{BufReader, Error as IoError, Read};
use std::time::Duration;

use url::Url;

use crate::parse::{parse_slice, Rules, BYTES_LIMIT};

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

///
#[derive(Debug, Clone)]
enum RobotsRules {
    Rules(Rules),
    Always(bool),
}

/// The `Robots` struct represents the set of directives related to
/// the specific `user-agent` in the provided `robots.txt` file.
#[derive(Debug, Clone)]
pub struct Robots {
    user_agent: String,
    rules: RobotsRules,
    sitemaps: Vec<Url>,
}

impl Robots {
    /// Creates a new `Robots` from the byte slice.
    pub fn from_slice(slice: &[u8], user_agent: &str) -> Self {
        let (user_agent, rules, sitemaps) = parse_slice(slice, user_agent);
        Self {
            user_agent,
            rules: RobotsRules::Rules(rules),
            sitemaps,
        }
    }

    /// Creates a new `Robots` from the generic reader.
    pub fn from_reader<R: Read>(reader: R, user_agent: &str) -> Result<Self, IoError> {
        let reader = reader.take(BYTES_LIMIT as u64);
        let mut reader = BufReader::new(reader);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let robots = buffer.as_slice();
        Ok(Self::from_slice(robots, user_agent))
    }

    /// Creates a new `Robots` from the `AccessResult`.
    pub fn from_access(access: AccessResult, user_agent: &str) -> Self {
        use AccessResult::*;
        match access {
            Successful(txt) => Self::from_slice(txt, user_agent),
            Redirect | Unavailable => Self::from_always(true, user_agent),
            Unreachable => Self::from_always(false, user_agent),
        }
    }

    /// Creates a new `Robots` from the always rule.
    pub fn from_always(always: bool, user_agent: &str) -> Self {
        Self {
            user_agent: user_agent.trim().to_lowercase(),
            rules: RobotsRules::Always(always),
            sitemaps: vec![],
        }
    }
}

impl Robots {
    /// Returns the longest matching user-agent.
    pub fn user_agent(&self) -> &str {
        &self.user_agent
    }

    /// Returns true if the path is allowed for the user-agent.
    /// NOTE: Expects relative path.
    pub fn is_allowed(&self, path: &str) -> bool {
        match &self.rules {
            RobotsRules::Rules(rules) => rules.is_allowed(path),
            RobotsRules::Always(always) => *always,
        }
    }

    /// Returns `Some(_)` if the site is fully allowed or disallowed.
    pub fn is_always(&self) -> Option<bool> {
        match &self.rules {
            RobotsRules::Rules(_) => None,
            RobotsRules::Always(always) => Some(*always),
        }
    }

    /// Returns the crawl-delay of the user-agent.
    pub fn crawl_delay(&self) -> Option<Duration> {
        match &self.rules {
            RobotsRules::Rules(rules) => rules.crawl_delay(),
            RobotsRules::Always(_) => None,
        }
    }

    /// Returns all sitemaps.
    pub fn sitemaps(&self) -> &Vec<Url> {
        &self.sitemaps
    }
}
