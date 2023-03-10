use std::collections::HashMap;
use std::io::{BufReader, Error as IoError, Read};
use std::time::Duration;

use url::Url;

use crate::internal::{into_directives, Directive, Rule, Rules, BYTES_LIMIT};
use crate::parse::{try_delay, try_rule, try_sitemaps, DEFAULT};

/// The `RobotsFile` struct represents all directives of
/// the provided `robots.txt` file.
#[derive(Debug, Clone)]
pub struct RobotsFile {
    rules: HashMap<String, Rules>,
    sitemaps: Vec<Url>,
}

type Data = (Vec<Rule>, Option<Duration>);

impl RobotsFile {
    /// Creates a new `RobotsFile` from the directives.
    fn from_directives(directives: &[Directive]) -> Self {
        let mut captured = [DEFAULT.to_string()].to_vec();
        let mut captures_group = false;

        let mut rules: HashMap<String, Data> = HashMap::new();
        let mut maps = Vec::new();

        for directive in directives {
            match directive {
                Directive::UserAgent(u) => {
                    let u = String::from_utf8(u.to_vec()).ok();
                    let u = u.map(|u| u.trim().to_lowercase());
                    if let Some(u) = u {
                        if !captures_group {
                            captured.clear();
                        }

                        captured.push(u);
                        continue;
                    }
                }

                Directive::Sitemap(u) => match try_sitemaps(u) {
                    Some(u) => maps.push(u),
                    None => continue,
                },

                Directive::Unknown(_) => continue,
                _ => captures_group = false,
            }

            match directive {
                Directive::Allow(u) => match try_rule(u, true) {
                    Some(_) => {}
                    None => continue,
                },

                Directive::Disallow(u) => match try_rule(u, false) {
                    Some(_) => {}
                    None => continue,
                },

                Directive::CrawlDelay(u) => match try_delay(u) {
                    Some(_) => {}
                    None => continue,
                },

                _ => unreachable!(),
            }
        }

        todo!()
    }

    /// Creates a new `RobotsFile` from the byte slice.
    pub fn from_slice(robots: &[u8]) -> Self {
        let directives = into_directives(robots);
        Self::from_directives(directives.as_slice())
    }

    /// Creates a new `RobotsFile` from the string slice.
    pub fn from_string(robots: &str) -> Self {
        let robots = robots.as_bytes();
        Self::from_slice(robots)
    }

    /// Creates a new `RobotsFile` from the generic reader.
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, IoError> {
        let reader = reader.take(BYTES_LIMIT as u64);
        let mut reader = BufReader::new(reader);

        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer)?;

        let robots = buffer.as_slice();
        Ok(Self::from_slice(robots))
    }
}

impl RobotsFile {
    ///
    fn find(&self, ua: &str) -> Option<Rules> {
        todo!()
    }

    /// Returns the longest matching user-agent.
    pub fn user_agent(&self, ua: &str) -> String {
        todo!()
    }

    /// Returns true if the path is allowed for the specified
    /// user-agent in the provided robots.txt file.
    pub fn is_match(&self, ua: &str, path: &str) -> bool {
        let r = self.find(ua).map(|ua| ua.is_match(path));
        r.unwrap_or(true)
    }

    /// Returns the crawl-delay of the longest matching user-agent.
    pub fn delay(&self, ua: &str) -> Option<Duration> {
        let r = self.find(ua).map(|ua| ua.delay());
        r.flatten()
    }

    /// Returns all sitemaps.
    pub fn sitemaps(&self) -> Vec<Url> {
        self.sitemaps.clone()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn foo() {}
}
