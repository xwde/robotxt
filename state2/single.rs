use std::io::Read;
use std::time::Duration;
use url::Url;

use crate::parse::{into_directives, Directive};
use crate::state::{Rule, Rules};
use crate::state2::DEFAULT_UA;

///
#[derive(Debug, Clone)]
pub struct Robots {
    agent: String,
    rules: Rules,
    sitemaps: Vec<Url>,
}

impl Robots {
    fn agents(directives: &Vec<Directive>, ua: &str) -> (String, Vec<u8>) {
        let agent = ua.trim().to_lowercase();
        let mut ans = (DEFAULT_UA.to_string(), DEFAULT_UA.as_bytes());
        for directive in directives {
            match directive.try_agent() {
                Some((s, u)) => match agent.starts_with(s.as_str()) {
                    true => ans = (s, u),
                    false => continue,
                },
                None => continue,
            }
        }

        (ans.0, ans.1.to_vec())
    }

    ///
    pub fn from_directives(directives: Vec<Directive>, ua: &str) -> Self {
        // Finds the longest matching user-agent.
        let agent = Self::agents(&directives, ua);

        // Captures everything before the first ua if default.
        let mut capturing = agent.1.eq(DEFAULT_UA.as_bytes());

        let mut rules = Vec::new();
        let mut delay = None;
        let mut sitemaps = Vec::new();

        use Directive::*;
        for directive in directives {
            match directive {
                UserAgent(u) => {
                    if !capturing {
                        capturing = u.eq(agent.1.as_slice());
                    }

                    continue;
                }

                Sitemap(_) => {
                    if let Some(u) = directive.try_sitemap() {
                        sitemaps.push(u)
                    }

                    continue;
                }

                Unknown(_) => continue,
                _ => capturing = false,
            }

            match directive {
                Allow(_) | Disallow(_) => match directive.try_rule() {
                    Some(u) => rules.push(u),
                    None => continue,
                },

                CrawlDelay(_) => {
                    let u = directive.try_delay();
                    delay = match delay.xor(u) {
                        None => delay.min(u),
                        Some(u) => Some(u),
                    };
                }

                _ => unreachable!(),
            }
        }

        Self {
            agent: agent.0,
            rules: Rules::new(rules, delay),
            sitemaps,
        }
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
    pub fn from_reader<R: Read>(reader: R, ua: &str) -> Self {
        todo!()
    }
}

impl Robots {
    ///
    pub fn is_match(&self, path: &str) -> bool {
        self.rules.is_match(path)
    }

    ///
    pub fn agent(&self) -> &String {
        &self.agent
    }

    ///
    pub fn rules(&self) -> Vec<Rule> {
        self.rules.rules()
    }

    ///
    pub fn delay(&self) -> Option<Duration> {
        self.rules.delay()
    }

    ///
    pub fn sitemaps(&self) -> &Vec<Url> {
        &self.sitemaps
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
