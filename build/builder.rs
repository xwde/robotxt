use std::collections::HashSet;
use std::error::Error;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::{Error as IoError, Write};
use std::time::Duration;

use url::Url;

/// The `Factory` struct represents a single `user-agent` group as per `RFC9309`.
#[derive(Debug, Clone)]
struct Section {
    // Usage of user-agent groups is discouraged.
    user_agents: String, // Vec<String>
    rules_disallow: HashSet<String>,
    rules_allow: HashSet<String>,
    crawl_delay: Option<Duration>,

    header: Option<String>,
    footer: Option<String>,
}

impl Section {
    ///
    pub fn new(user_agent: &str) -> &mut Self {
        todo!()
    }

    ///
    pub fn allow(&mut self) -> &mut Self {
        todo!()
    }

    ///
    pub fn disallow(&mut self) -> &mut Self {
        todo!()
    }

    ///
    pub fn crawl_delay(&mut self) -> &mut Self {
        todo!()
    }

    ///
    pub fn build<W: Write>(&self, writer: W) -> Self {
        todo!()
    }
}

/// See [Factory::build].
#[derive(Debug)]
pub enum FactoryError {
    IoError(IoError),
}

impl Display for FactoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        todo!()
    }
}

impl From<IoError> for FactoryError {
    fn from(error: IoError) -> Self {
        Self::IoError(error)
    }
}

impl Error for FactoryError {}

/// The `Factory` struct represents TODO
#[derive(Debug, Default)]
pub struct Factory {}

impl Factory {
    ///
    pub fn new() -> Self {
        todo!()
    }

    ///
    pub fn header(&mut self, header: &str) -> &mut Self {
        todo!()
    }

    ///
    pub fn footer(&mut self, footer: &str) -> &mut Self {
        todo!()
    }

    ///
    pub fn group(&mut self, section: Section) -> &mut Self {
        todo!()
    }

    ///
    pub fn sitemap(&mut self, sitemaps: Url) -> &mut Self {
        todo!()
    }

    ///
    pub fn build<W: Write>(&self, writer: W) {
        // writer.wr
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufWriter, stdout};
    use super::*;

    #[test]
    fn test() {
        let writer = Vec::new();

        Factory::new()
            .header("header text")
            .sitemap(Url::parse("").unwrap())
            .footer("footer text")
            .build(writer);
    }
}
