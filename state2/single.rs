use std::time::Duration;
use url::Url;

use crate::parse::{into_directives, Directive};
use crate::state::Rules;
use crate::DEFAULT;

///
#[derive(Debug, Clone)]
pub struct Robots {
    agent: String,
    rules: Rules,
    sitemaps: Vec<Url>,
}

impl Robots {
    ///
    pub fn agent(&self) -> &String {
        &self.agent
    }

    ///
    pub fn is_match(&self, path: &str) -> bool {
        self.rules.is_match(path)
    }

    ///
    pub fn sitemaps(&self) -> &Vec<Url> {
        &self.sitemaps
    }

    ///
    pub fn delay(&self) -> Option<Duration> {
        self.rules.delay()
    }
}

impl Robots {
    pub fn from_directives(directives: Vec<Directive>, agent: &str) -> Self {
        // Collects all sitemaps.
        let sitemaps = directives.iter().filter_map(|u| u.try_sitemap());
        let sitemaps = sitemaps.collect();

        let agent = agent.trim().to_lowercase();
        // Finds all matching user-agents.
        let agent = directives.iter().filter_map(|u| match *u {
            UserAgent(u) => {
                let u_str = String::from_utf8(u.to_vec()).ok()?;
                let u_str = u_str.trim().to_lowercase();
                match agent.starts_with(u_str.as_str()) {
                    true => Some((u_str, u)),
                    false => None,
                }
            }
            _ => None,
        });

        // Finds the longest matching user-agent.
        let agent: Vec<_> = agent.collect();
        let agent = agent.iter().max_by(|l, r| l.0.len().cmp(&r.0.len()));
        let default = (DEFAULT.to_string(), DEFAULT.as_bytes());
        let (agent, agent_u8) = agent.unwrap_or(&default);

        let mut rules = Vec::new();
        let mut delay = None;

        // Captures everything before the first ua if default.
        let mut capturing = (*agent_u8).eq(DEFAULT.as_bytes());

        use Directive::*;
        for directive in directives {
            match directive {
                UserAgent(u) => {
                    if !capturing {
                        capturing = u.eq(*agent_u8);
                    }

                    continue;
                }

                Sitemap(_) => continue,
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
            agent: agent.to_string(),
            rules: Rules::new(rules, delay),
            sitemaps,
        }
    }

    pub fn from_slice(robots: &[u8], agent: &str) -> Self {
        let directives = into_directives(robots);
        Self::from_directives(directives, agent)
    }

    pub fn from_string(robots: &str, agent: &str) -> Self {
        let robots = robots.as_bytes();
        Self::from_slice(robots, agent)
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
