use std::time::Duration;

use crate::parse::{normalize_path, Rule};

/// The `Rules` struct provides a convenient and efficient storage for
/// the data associated with certain user-agent for further matching.
#[derive(Debug, Clone)]
pub struct Rules {
    rules: Vec<Rule>,
    delay: Option<Duration>,
}

impl Rules {
    /// Creates a new `Rules` with the specified rules and delay.
    pub fn new(rules: Vec<Rule>, delay: Option<Duration>) -> Self {
        let mut rules = rules;

        // Rules are sorted by length and permission i.e.
        // 5a > 4a, 5a > 5d, 5d > 4a
        rules.sort();
        Self { rules, delay }
    }

    /// Returns true if the path is allowed for this set of rules.
    pub fn is_match(&self, path: &str) -> bool {
        let path = normalize_path(path);

        if path.eq("/robots.txt") {
            return true;
        }

        for rule in &self.rules {
            if rule.is_match(path.as_str()) {
                return rule.is_allowed();
            }
        }

        true
    }

    /// Returns the specified crawl-delay.
    pub fn delay(&self) -> Option<Duration> {
        self.delay
    }
}

#[cfg(test)]
mod precedence {
    use super::*;

    #[test]
    fn simple() {
        let allow = Rule::new("/p", true).unwrap();
        let disallow = Rule::new("/", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(rules.is_match("/page"));
    }

    #[test]
    fn restrictive() {
        let allow = Rule::new("/folder", true).unwrap();
        let disallow = Rule::new("/folder", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(rules.is_match("/folder/page"));
    }

    #[test]
    fn restrictive2() {
        let allow = Rule::new("/page", true).unwrap();
        let disallow = Rule::new("/*.ph", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(rules.is_match("/page.php5"));
    }

    #[test]
    fn longer() {
        let allow = Rule::new("/page", true).unwrap();
        let disallow = Rule::new("/*.htm", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(!rules.is_match("/page.htm"));
    }

    #[test]
    fn specific() {
        let allow = Rule::new("/$", true).unwrap();
        let disallow = Rule::new("/", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(rules.is_match("/"));
    }

    #[test]
    fn specific2() {
        let allow = Rule::new("/$", true).unwrap();
        let disallow = Rule::new("/", false).unwrap();
        let rules = Rules::new(vec![allow, disallow], None);

        assert!(!rules.is_match("/page.htm"));
    }
}
