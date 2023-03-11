use std::cmp::min;
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

///
const DEFAULT: &str = "*";

/// Finds the longest matching user-agent and
/// if the parser should check non-assigned rules.
fn find_agent(directives: &[Directive], user_agent: &str) -> (String, bool) {
    // Collects all uas.
    let uas = directives.iter().filter_map(|ua2| match ua2 {
        Directive::UserAgent(ua2) => String::from_utf8(ua2.to_vec()).ok(),
        _ => None,
    });

    // Filters out non-acceptable uas.
    let ua = user_agent.trim().to_lowercase();
    let uas = uas.map(|ua2| ua2.trim().to_lowercase());
    let uas = uas.filter(|ua2| ua.starts_with(ua2.as_str()));

    // Finds the longest ua in the acceptable pool.
    let uas = uas.max_by(|lhs, rhs| lhs.len().cmp(&rhs.len()));
    let uas = uas.unwrap_or(DEFAULT.to_string());

    // Determines if it should check non-assigned rules.
    let default = uas.eq(DEFAULT);
    (uas, default)
}

///
fn parse_directives(directives: &[Directive], user_agent: &str) -> (String, Rules, Vec<Url>) {
    let (user_agent, mut captures_rules) = find_agent(directives, user_agent);
    let mut captures_group = false;

    let mut rules = Vec::new();
    let mut delay = None;
    let mut sitemaps = Vec::new();

    for directive in directives {
        match directive {
            Directive::UserAgent(u) => {
                if let Some(u) = try_agent(u) {
                    if !captures_group || !captures_rules {
                        captures_rules = u.eq(&user_agent);
                    }
                }

                captures_group = true;
                continue;
            }

            Directive::Sitemap(u) => {
                if let Some(u) = try_sitemaps(u) {
                    sitemaps.push(u);
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

    let rules = Rules::new(rules, delay);
    (user_agent, rules, sitemaps)
}

///
pub fn parse_slice(robots: &[u8], user_agent: &str) -> (String, Rules, Vec<Url>) {
    // Limits the input to 500 kibibytes.
    let limit = min(robots.len(), BYTES_LIMIT);
    let robots = &robots[0..limit];

    // Replaces '\x00' with '\n'.
    let robots = robots.iter().map(|u| match u {
        b'\x00' => b'\n',
        v => *v,
    });

    let robots: Vec<_> = robots.collect();
    let robots = robots.as_slice();

    let directives = into_directives(robots);
    parse_directives(directives.as_slice(), user_agent)
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
        let (_, r, _) = parse_directives(DIRECTIVES, "bot-robotxt");

        // Matches:
        assert!(r.is_allowed("/1"));

        // Doesn't match:
        assert!(!r.is_allowed("/2"));
        assert!(!r.is_allowed("/3"));
    }

    #[test]
    fn strict() {
        let (_, r, _) = parse_directives(DIRECTIVES, "bot");

        // Matches:
        assert!(r.is_allowed("/3"));

        // Doesn't match:
        assert!(!r.is_allowed("/1"));
        assert!(!r.is_allowed("/2"));
    }

    #[test]
    fn missing() {
        let (_, r, _) = parse_directives(DIRECTIVES, "super-bot");

        // Matches:
        assert!(r.is_allowed("/2"));

        // Doesn't match:
        assert!(!r.is_allowed("/1"));
        assert!(!r.is_allowed("/3"));
    }

    #[test]
    fn partial() {
        let (_, r, _) = parse_directives(DIRECTIVES, "bot-super");

        // Matches:
        assert!(r.is_allowed("/3"));

        // Doesn't match:
        assert!(!r.is_allowed("/1"));
        assert!(!r.is_allowed("/2"));
    }
}
