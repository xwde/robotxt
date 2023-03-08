use std::collections::HashMap;
use std::io::Read;
use std::time::Duration;

use url::Url;

use crate::parse::{into_directives, Directive};
use crate::state::{Rule, Rules};

///
#[derive(Debug, Clone)]
pub struct RobotsFile {
    rules: HashMap<String, Rules>,
    sitemaps: Vec<Url>,
}

impl RobotsFile {
    ///
    pub fn from_directives(directives: Vec<Directive>) -> Self {
        //
        let mut sitemaps = Vec::<Url>::new();
        let mut rules = HashMap::<Vec<u8>, Vec<Rule>>::new();
        let mut delay = HashMap::<Vec<u8>, Duration>::new();

        let mut captured: Vec<Vec<u8>> = [b"*".to_vec()].into();
        let mut capturing = false;

        for directive in directives {
            match directive {
                Directive::UserAgent(u) => {
                    if capturing {
                        captured.push(u.to_vec());
                    } else {
                        captured.clear();
                        capturing = true;
                    }

                    continue;
                }

                Directive::Sitemap(_) => {
                    if let Some(u) = directive.try_sitemap() {
                        sitemaps.push(u);
                    }

                    continue;
                }

                Directive::Unknown(_) => continue,
                _ => capturing = false,
            }

            match directive {
                Directive::Allow(_) => {}
                Directive::Disallow(_) => {}
                Directive::CrawlDelay(_) => {}
                _ => unreachable!(),
            }
        }

        todo!()
    }

    ///
    pub fn from_slice(robots: &[u8]) -> Self {
        let directives = into_directives(robots);
        Self::from_directives(directives)
    }

    ///
    pub fn from_string(robots: &str) -> Self {
        let robots = robots.as_bytes();
        Self::from_slice(robots)
    }

    ///
    pub fn from_reader<R: Read>(reader: R, ua: &str) -> Self {
        todo!()
    }
}

impl RobotsFile {
    ///
    pub fn is_match(&self, ua: &str, path: &str) -> bool {
        let r = self.find(ua).map(|ua| ua.is_match(path));
        r.unwrap_or(true)
    }

    ///
    pub fn rules(&self, ua: &str) -> Vec<Rule> {
        let r = self.find(ua).map(|ua| ua.rules());
        r.unwrap_or(Vec::new())
    }

    ///
    pub fn delay(&self, ua: &str) -> Option<Duration> {
        let r = self.find(ua).map(|ua| ua.delay());
        r.flatten()
    }

    ///
    pub fn sitemaps(&self) -> Vec<Url> {
        self.sitemaps.clone()
    }

    ///
    fn find(&self, ua: &str) -> Option<Rules> {
        todo!()
    }
}
