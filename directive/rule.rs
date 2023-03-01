use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::sync::OnceLock;

use regex::{Error as RegexError, Regex, RegexBuilder};

/// An error type indicating that a `Wildcard` could not be correctly parsed.
#[derive(Debug, Clone)]
pub enum WildcardError {
    RegexError(RegexError),
}

impl Display for WildcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::RegexError(e) => Display::fmt(e, f),
        }
    }
}

impl From<RegexError> for WildcardError {
    fn from(error: RegexError) -> Self {
        Self::RegexError(error)
    }
}
impl Error for WildcardError {}

/// The `Wildcard` struct provides efficient pattern matching for the `Rule` struct.
#[derive(Debug, Clone)]
enum Wildcard {
    Ending(String),
    Universal(String),
    Both(Regex),
}

impl Wildcard {
    /// Creates a new `Wildcard` with the specified pattern or
    /// returns `None` if the specified pattern does not contain any wildcard.
    pub fn new(pattern: &str) -> Result<Option<Wildcard>, WildcardError> {
        if !pattern.contains('$') && !pattern.contains('*') {
            return Ok(None);
        }

        // if pattern.contains('$') && !pattern.contains('*') {
        //     todo!()
        // }

        // if pattern.contains('*') && !pattern.contains('$') {
        //     todo!()
        // }

        static STAR_KILLER: OnceLock<Regex> = OnceLock::new();
        let star_killer = STAR_KILLER.get_or_init(|| Regex::new(r"\*+").unwrap());

        let regex = star_killer.replace_all(pattern, "*");
        let regex = '^'.to_string()
            + &regex::escape(&regex)
                .replace("\\*", ".*")
                .replace("\\$", "$");
        let regex = RegexBuilder::new(&regex)
            .dfa_size_limit(42 * (1 << 10))
            .size_limit(42 * (1 << 10))
            .build()?;

        Ok(Some(Self::Both(regex)))
    }

    /// Returns true if the path matches the pattern of this wildcard.
    pub fn is_match(&self, path: &str) -> bool {
        match &self {
            Self::Ending(_) => todo!(),
            Self::Universal(_) => todo!(),
            Self::Both(r) => r.is_match(path),
        }
    }
}

/// The `UserAgentRule` structs provides an efficient way to find
/// the longest match (with a allow priority) in any sortable container.
#[derive(Debug, Clone)]
pub struct UserAgentRule {
    pattern: String,
    allow: bool,
    wildcard: Option<Wildcard>,
}

impl UserAgentRule {
    /// Creates a new `UserAgentRule` with the specified pattern and permission.
    pub fn new(pattern: &str, allow: bool) -> Result<Self, WildcardError> {
        let pattern = match pattern.starts_with('/') {
            false => '/'.to_string() + pattern,
            true => pattern.to_string(),
        };

        let wildcard = Wildcard::new(pattern.as_str())?;

        Ok(Self {
            pattern,
            allow,
            wildcard,
        })
    }

    /// Returns a string slice containing the original pattern.
    pub fn pattern(&self) -> &str {
        self.pattern.as_str()
    }

    /// Returns true if the path matches the pattern of this rule.
    pub fn is_match(&self, path: &str) -> bool {
        match &self.wildcard {
            None => path.starts_with(&self.pattern),
            Some(w) => w.is_match(path),
        }
    }

    /// Returns the permission of this rule.
    pub fn is_allow(&self) -> bool {
        self.allow
    }
}

impl PartialEq<Self> for UserAgentRule {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.eq(&other.pattern)
    }
}

impl Eq for UserAgentRule {}

impl PartialOrd<Self> for UserAgentRule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for UserAgentRule {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.pattern.len().cmp(&self.pattern.len()) {
            Ordering::Equal => other.allow.cmp(&self.allow),
            v => v,
        }
    }
}

#[cfg(test)]
mod matching {
    use super::*;

    #[test]
    fn root_none() {
        let _r = UserAgentRule::new("/", true).unwrap();
        // TODO
    }

    #[test]
    fn root_universal() {
        let _r = UserAgentRule::new("/*", true).unwrap();
        // TODO
    }

    #[test]
    fn root_ending() {
        let _r = UserAgentRule::new("/$", true).unwrap();
        // TODO
    }

    #[test]
    fn simple() {
        let r = UserAgentRule::new("/fish", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish"));
        assert!(r.is_match("/fish.html"));
        assert!(r.is_match("/fish/salmon.html"));
        assert!(r.is_match("/fishheads"));
        assert!(r.is_match("/fishheads/yummy.html"));
        assert!(r.is_match("/fish.php?id=anything"));

        // Doesn't match:
        assert!(!r.is_match("/Fish.asp"));
        assert!(!r.is_match("/catfish"));
        assert!(!r.is_match("/?id=fish"));
        assert!(!r.is_match("/desert/fish"));
    }

    #[test]
    fn folder() {
        let r = UserAgentRule::new("/fish/", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish/"));
        assert!(r.is_match("/fish/?id=anything"));
        assert!(r.is_match("/fish/salmon.htm"));

        // Doesn't match:
        assert!(!r.is_match("/fish"));
        assert!(!r.is_match("/fish.html"));
        assert!(!r.is_match("/animals/fish/"));
        assert!(!r.is_match("/Fish/Salmon.asp"));
    }

    #[test]
    fn universal_end() {
        let r = UserAgentRule::new("/fish*", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish"));
        assert!(r.is_match("/fish.html"));
        assert!(r.is_match("/fish/salmon.html"));
        assert!(r.is_match("/fishheads"));
        assert!(r.is_match("/fishheads/yummy.html"));
        assert!(r.is_match("/fish.php?id=anything"));

        // Doesn't match:
        assert!(!r.is_match("/Fish.asp"));
        assert!(!r.is_match("/catfish"));
        assert!(!r.is_match("/?id=fish"));
        assert!(!r.is_match("/desert/fish"));
    }

    #[test]
    fn universal_mid() {
        let r = UserAgentRule::new("/*.php", true).unwrap();

        // Matches:
        assert!(r.is_match("/index.php"));
        assert!(r.is_match("/filename.php"));
        assert!(r.is_match("/folder/filename.php"));
        assert!(r.is_match("/folder/filename.php?parameters"));
        assert!(r.is_match("/folder/any.php.file.html"));
        assert!(r.is_match("/filename.php/"));

        // Doesn't match:
        assert!(!r.is_match("/"));
        assert!(!r.is_match("/windows.PHP"));
    }

    #[test]
    fn universal_mid2() {
        let r = UserAgentRule::new("/fish*.php", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish.php"));
        assert!(r.is_match("/fishheads/catfish.php?parameters"));

        // Doesn't match:
        assert!(!r.is_match("/Fish.PHP"));
    }

    #[test]
    fn both_wildcards() {
        let r = UserAgentRule::new("/*.php$", true).unwrap();

        // Matches:
        assert!(r.is_match("/filename.php"));
        assert!(r.is_match("/folder/filename.php"));

        // Doesn't match:
        assert!(!r.is_match("/filename.php?parameters"));
        assert!(!r.is_match("/filename.php/"));
        assert!(!r.is_match("/filename.php5"));
        assert!(!r.is_match("/windows.PHP"));
    }
}
