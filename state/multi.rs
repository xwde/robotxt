use std::collections::HashMap;
use std::io::{BufReader, Error as IoError, Read};
use std::time::Duration;

use url::Url;

use crate::parse::{into_directives, Directive, Rules, BYTES_LIMIT};

/// The `RobotsFile` struct represents all directives of the
/// provided `robots.txt` file.
#[derive(Debug, Clone)]
pub struct RobotsFile {
    rules: HashMap<String, Rules>,
    sitemaps: Vec<Url>,
}

impl RobotsFile {
    /// Creates a new `RobotsFile` from the directives.
    fn from_directives(directives: Vec<Directive>) -> Self {
        for directive in directives {}

        todo!()
    }

    /// Creates a new `RobotsFile` from the byte slice.
    pub fn from_slice(robots: &[u8]) -> Self {
        let directives = into_directives(robots);
        Self::from_directives(directives)
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
    pub fn ua(&self, ua: &str) -> String {
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
