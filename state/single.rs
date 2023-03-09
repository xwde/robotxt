use std::io::{BufReader, Error as IoError, Read};
use std::time::Duration;

use url::Url;

use crate::parse::{into_directives, Directive, BYTES_LIMIT};
use crate::parse::{Rule, Rules};
use crate::state::DEFAULT;
use crate::state::{try_delay, try_rule, try_sitemaps};

///
#[derive(Debug, Clone)]
pub struct Robots {
    ua: String,
    rules: Rules,
    maps: Vec<Url>,
}

impl Robots {
    ///
    fn extract_sitemaps(directives: &[Directive]) -> Vec<Url> {
        todo!()
    }

    /// Finds the longest matching user-agent and if it should check non-assigned rules.
    fn find_agent(directives: &[Directive], ua: &str) -> (String, bool) {
        // Collects all uas.
        let uas = directives.iter().filter_map(|ua2| match ua2 {
            Directive::UserAgent(ua2) => String::from_utf8(ua2.to_vec()).ok(),
            _ => None,
        });

        // Filters out non-acceptable uas.
        let ua = ua.trim().to_lowercase();
        let uas = uas.map(|ua2| (ua2.trim().to_lowercase(), ua2));
        let uas = uas.filter_map(|ua2| match ua.starts_with(ua2.0.as_str()) {
            true => Some(ua2.1),
            false => None,
        });

        // Finds the longest ua in the acceptable pool.
        let uas = uas.max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()));
        let uas = uas.unwrap_or(DEFAULT.to_string());

        // Determines if it should check non-assigned rules.
        let default = uas.eq(DEFAULT);
        (uas, default)
    }

    ///
    pub fn from_directives(directives: Vec<Directive>, ua: &str) -> Self {
        let (ua, mut captures) = Self::find_agent(directives.as_slice(), ua);
        let mut in_group = false;

        let mut rules = Vec::new();
        let mut delay = None;
        let mut maps = Vec::new();

        for directive in directives {
            match directive {
                Directive::UserAgent(u) => {
                    if !in_group || !captures {
                        captures = u == ua.as_bytes()
                    }

                    in_group = true;
                    continue;
                }

                Directive::Sitemap(u) => match try_sitemaps(u) {
                    Some(u) => maps.push(u),
                    None => continue,
                },

                Directive::Unknown(_) => continue,
                _ => in_group = false,
            }

            if !captures {
                continue;
            }

            match directive {
                Directive::Allow(u) => match try_rule(u, true) {
                    Some(u) => rules.push(u),
                    None => continue,
                },

                Directive::Disallow(u) => match try_rule(u, false) {
                    Some(u) => rules.push(u),
                    None => continue,
                },

                Directive::CrawlDelay(u) => match try_delay(u) {
                    Some(u) => delay = Some(u), // TODO min
                    None => continue,
                },

                _ => unreachable!(),
            }
        }

        let rules = Rules::new(rules, delay);
        Self { ua, rules, maps }
    }

    ///
    pub fn from_slice(robots: &[u8], ua: &str) -> Self {
        let directives = into_directives(robots);
        Self::from_directives(directives, ua)
    }

    ///
    pub fn from_string(robots: &str, ua: &str) -> Self {
        let robots = robots.as_bytes();
        Self::from_slice(robots, ua)
    }

    ///
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
    /// Returns true if the path is allowed for the provided robots.txt file.
    pub fn is_match(&self, path: &str) -> bool {
        self.rules.is_match(path)
    }

    ///
    pub fn ua(&self) -> &String {
        &self.ua
    }

    ///
    pub fn rules(&self) -> Vec<Rule> {
        self.rules.rules()
    }

    ///
    pub fn delay(&self) -> Option<Duration> {
        self.rules.delay()
    }

    /// Returns all sitemaps specified in the provided robots.txt file.
    pub fn sitemaps(&self) -> &Vec<Url> {
        &self.maps
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::*;

    #[test]
    fn r2oo() {
        let r = read_to_string("./sample/google.txt").unwrap();
        let r = Robots::from_string(r.as_str(), "AdsBot-Google");
        dbg!(r);
    }
}
