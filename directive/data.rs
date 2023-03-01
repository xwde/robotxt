use std::time::Duration;

use crate::directive::UserAgentRule;

#[derive(Debug)]
pub struct UserAgentData {
    rules: Vec<UserAgentRule>,
    delay: Option<Duration>,
}

impl UserAgentData {
    /// Creates a new `UserAgentData` with the specified rules and delay.
    pub fn new(rules: Vec<UserAgentRule>, delay: Option<Duration>) -> Self {
        let mut rules = rules;
        rules.sort();
        Self { rules, delay }
    }

    /// Returns true if the path is allowed for this user agent.
    pub fn is_match(&self, path: &str) -> bool {
        for rule in &self.rules {
            if rule.is_match(path) {
                return rule.is_allow();
            }
        }

        true
    }

    ///
    pub fn delay(&self) -> Option<Duration> {
        self.delay
    }
}

#[cfg(test)]
mod precedence {
    use crate::directive::UserAgentData;
    use crate::directive::UserAgentRule;

    #[test]
    fn simple() {
        let allow = UserAgentRule::new("/p", true).unwrap();
        let disallow = UserAgentRule::new("/", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(rules.is_match("/page"));
    }

    #[test]
    fn restrictive() {
        let allow = UserAgentRule::new("/folder", true).unwrap();
        let disallow = UserAgentRule::new("/folder", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(rules.is_match("/folder/page"));
    }

    #[test]
    fn restrictive2() {
        let allow = UserAgentRule::new("/page", true).unwrap();
        let disallow = UserAgentRule::new("/*.ph", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(rules.is_match("/page.php5"));
    }

    #[test]
    fn longer() {
        let allow = UserAgentRule::new("/page", true).unwrap();
        let disallow = UserAgentRule::new("/*.htm", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(!rules.is_match("/page.htm"));
    }

    #[test]
    fn specific() {
        let allow = UserAgentRule::new("/$", true).unwrap();
        let disallow = UserAgentRule::new("/", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(rules.is_match("/"));
    }

    #[test]
    fn specific2() {
        let allow = UserAgentRule::new("/$", true).unwrap();
        let disallow = UserAgentRule::new("/", false).unwrap();
        let rules = UserAgentData::new(vec![allow, disallow], None);

        assert!(!rules.is_match("/page.htm"));
    }
}
