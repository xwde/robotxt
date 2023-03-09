use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};

use once_cell::sync::OnceCell;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use regex::{Error as RegexError, Regex, RegexBuilder};

/// An error type indicating that a `Wildcard` could not be parsed correctly.
#[derive(Debug, Clone)]
pub enum WildcardError {
    // EndingTooMany(usize),
    // EndingPosition(usize),
    Regex(RegexError),
}

impl Display for WildcardError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match &self {
            Self::Regex(e) => Display::fmt(e, f),
        }
    }
}

impl From<RegexError> for WildcardError {
    fn from(error: RegexError) -> Self {
        Self::Regex(error)
    }
}

impl Error for WildcardError {}

/// The `Wildcard` struct provides efficient pattern matching for wildcards.
#[derive(Debug, Clone)]
pub enum Wildcard {
    // Ending(String),
    // Universal(String),
    Both(Regex),
}

impl Wildcard {
    /// Creates a new `Wildcard` with the specified pattern or returns
    /// `None` if the specified pattern does not contain any wildcard.
    pub fn new(pattern: &str) -> Result<Option<Self>, WildcardError> {
        if !pattern.contains('$') && !pattern.contains('*') {
            return Ok(None);
        }

        // TODO only end of pattern wildcard
        // if pattern.contains('$') && !pattern.contains('*') { }

        // TODO only universal wildcard
        // if pattern.contains('*') && !pattern.contains('$') { }

        let regex = '^'.to_string()
            + &regex::escape(pattern)
                .replace("\\*", ".*")
                .replace("\\$", "$");
        let regex = RegexBuilder::new(&regex)
            .dfa_size_limit(42 * (1 << 10))
            .size_limit(42 * (1 << 10))
            .build()?;

        Ok(Some(Self::Both(regex)))
    }

    /// Returns true if the path matches the wildcard pattern.
    pub fn is_match(&self, path: &str) -> bool {
        match &self {
            // Self::Ending(_) => todo!(),
            // Self::Universal(_) => todo!(),
            Self::Both(r) => r.is_match(path),
        }
    }
}

/// The `Rule` struct provides a convenient and efficient way to process
/// and to match robots.txt provided patterns with relative paths.
#[derive(Debug, Clone)]
pub struct Rule {
    pattern: String,
    allow: bool,
    wildcard: Option<Wildcard>,
}

impl Rule {
    /// Creates a new `Rule` with the specified pattern and permission.
    pub fn new(pattern: &str, allow: bool) -> Result<Self, WildcardError> {
        let pattern = Self::normalize(pattern);
        let wildcard = Wildcard::new(pattern.as_str())?;

        Ok(Self {
            pattern,
            allow,
            wildcard,
        })
    }

    /// Returns true if the path matches the pattern.
    pub fn is_match(&self, path: &str) -> bool {
        let path = Self::normalize(path);
        match &self.wildcard {
            None => path.starts_with(self.pattern.as_str()),
            Some(wildcard) => wildcard.is_match(path.as_str()),
        }
    }

    /// Returns true if allowed.
    pub fn is_allowed(&self) -> bool {
        self.allow
    }

    /// Returns the prefixed & percent-encoded path.
    fn normalize(path: &str) -> String {
        // TODO replace once_cell with std::sync::OnceLock once stable
        static FRAGMENT: OnceCell<AsciiSet> = OnceCell::new();
        let fragment = FRAGMENT.get_or_init(|| CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>'));
        let path = utf8_percent_encode(path, fragment).to_string();

        match path.starts_with('/') {
            false => '/'.to_string() + path.as_str(),
            true => path,
        }
    }
}

impl PartialEq<Self> for Rule {
    fn eq(&self, other: &Self) -> bool {
        self.pattern.eq(&other.pattern)
    }
}

impl Eq for Rule {}

impl PartialOrd<Self> for Rule {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rule {
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
        let r = Rule::new("/", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish"));
    }

    #[test]
    fn root_universal() {
        let r = Rule::new("/*", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish"));
    }

    #[test]
    fn root_ending() {
        let r = Rule::new("/$", true).unwrap();

        // Matches:
        assert!(r.is_match("/"));

        // Doesn't match:
        assert!(!r.is_match("/fish"));
    }

    #[test]
    fn simple() {
        let r = Rule::new("/fish", true).unwrap();

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
        let r = Rule::new("/fish/", true).unwrap();

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
        let r = Rule::new("/fish*", true).unwrap();

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
        let r = Rule::new("/*.php", true).unwrap();

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
        let r = Rule::new("/fish*.php", true).unwrap();

        // Matches:
        assert!(r.is_match("/fish.php"));
        assert!(r.is_match("/fishheads/catfish.php?parameters"));

        // Doesn't match:
        assert!(!r.is_match("/Fish.PHP"));
    }

    #[test]
    fn both_wildcards() {
        let r = Rule::new("/*.php$", true).unwrap();

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